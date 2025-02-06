pub mod ipc;
pub mod webview;

use std::{
    borrow::Cow,
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    ops::Deref,
};

use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyString, IntoPyObject};
use pyo3_utils::{
    py_wrapper::{PyWrapper, PyWrapperSemverExt as _, PyWrapperT0, PyWrapperT2},
    ungil::UnsafeUngilExt,
};
use tauri::{Emitter as _, Listener as _, Manager as _};

use crate::{tauri_runtime::Runtime, utils::TauriError};

/// see also: [tauri::RunEvent]
//
// TODO, PERF: maybe should use [PyString]?
#[pyclass(frozen)]
#[non_exhaustive]
pub enum RunEvent {
    Exit(),
    #[non_exhaustive]
    ExitRequested {
        code: Option<i32>,
        // TODO, XXX, FIXME: `ExitRequestApi` is a private type in `tauri`,
        // we need create a issue to `tauri`, or we cant implement this.
        // api: ExitRequestApi,
    },
    #[non_exhaustive]
    WindowEvent {
        label: String,
        // TODO:
        // event: WindowEvent,
    },
    #[non_exhaustive]
    WebviewEvent {
        label: String,
        // TODO:
        // event: WebviewEvent,
    },
    Ready(),
    Resumed(),
    MainEventsCleared(),
    MenuEvent(/* TODO: tauri::menu::MenuEvent */),
    // TODO:
    // TrayIconEvent(tauri::tray::TrayIconEvent),
}

impl RunEvent {
    fn new(value: tauri::RunEvent) -> Self {
        match value {
            tauri::RunEvent::Exit => Self::Exit(),
            tauri::RunEvent::ExitRequested {
                code, /* TODO */ ..
            } => Self::ExitRequested { code },
            tauri::RunEvent::WindowEvent {
                label, /* TODO */ ..
            } => Self::WindowEvent { label },
            tauri::RunEvent::WebviewEvent {
                label, /* TODO */ ..
            } => Self::WebviewEvent { label },
            tauri::RunEvent::Ready => Self::Ready(),
            tauri::RunEvent::Resumed => Self::Resumed(),
            tauri::RunEvent::MainEventsCleared => Self::MainEventsCleared(),
            tauri::RunEvent::MenuEvent(/* TODO */ _) => Self::MenuEvent(),
            // TODO: tauri::RunEvent::TrayIconEvent,
            event => unimplemented!("Please make a issue for unimplemented RunEvent: {event:?}"),
        }
    }
}

/// You can get the global singleton [Py]<[AppHandle]> using [PyAppHandleExt].
#[pyclass(frozen)]
#[non_exhaustive]
// NOTE: Do not use [PyWrapperT2], otherwise the global singleton [PyAppHandle]
// will be consumed and cannot be used;
// If you really need ownership of [tauri::AppHandle], you can use [tauri::AppHandle::clone].
pub struct AppHandle(pub PyWrapper<PyWrapperT0<tauri::AppHandle<Runtime>>>);

impl AppHandle {
    fn new(app_handle: tauri::AppHandle<Runtime>) -> Self {
        Self(PyWrapper::new0(app_handle))
    }
}

struct PyAppHandle(Py<AppHandle>);

impl PyAppHandle {
    fn new(py_app_handle: Py<AppHandle>) -> Self {
        Self(py_app_handle)
    }
}

impl Deref for PyAppHandle {
    type Target = Py<AppHandle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// This error indicates that the app was not initialized using [App::try_build],
/// i.e. it was not created by pytauri.
#[derive(Debug)]
pub struct PyAppHandleStateError;

impl Display for PyAppHandleStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to get `PyAppHandle` from state, maybe this app was not created by pytauri"
        )
    }
}

impl Error for PyAppHandleStateError {}

impl From<PyAppHandleStateError> for PyErr {
    fn from(value: PyAppHandleStateError) -> Self {
        PyRuntimeError::new_err(format!("{value}"))
    }
}

pub type PyAppHandleStateResult<T> = Result<T, PyAppHandleStateError>;

/// You can use this trait to get the global singleton [Py]<[AppHandle]>.
pub trait PyAppHandleExt<R: tauri::Runtime>: tauri::Manager<R> {
    /// # Panics
    ///
    /// Panics if [PyAppHandleExt::try_py_app_handle] returns an error.
    fn py_app_handle(&self) -> impl Deref<Target = Py<AppHandle>> {
        self.try_py_app_handle().unwrap()
    }

    fn try_py_app_handle(&self) -> PyAppHandleStateResult<impl Deref<Target = Py<AppHandle>>> {
        self.try_state::<PyAppHandle>()
            .map(|state| state.inner().deref())
            .ok_or(PyAppHandleStateError)
    }
}

impl<R: tauri::Runtime, T: tauri::Manager<R>> PyAppHandleExt<R> for T {}

#[pyclass(frozen, unsendable)]
#[non_exhaustive]
pub struct App(pub PyWrapper<PyWrapperT2<tauri::App<Runtime>>>);

impl App {
    #[cfg(feature = "__private")]
    pub fn try_build(py: Python<'_>, app: tauri::App<Runtime>) -> PyResult<Self> {
        let app_handle = AppHandle::new(app.handle().to_owned());
        let py_app_handle = PyAppHandle::new(app_handle.into_pyobject(py)?.unbind());
        // if false, there has already state set for the app instance.
        if !app.manage(py_app_handle) {
            unreachable!(
                "`PyAppHandle` is private, so it is impossible for other crates to manage it"
            )
        }
        Ok(Self(PyWrapper::new2(app)))
    }

    fn py_cb_to_rs_cb(
        callback: PyObject,
    ) -> impl FnMut(&tauri::AppHandle<Runtime>, tauri::RunEvent) {
        move |app_handle, run_event| {
            let py_app_handle = app_handle.py_app_handle();
            let py_run_event = RunEvent::new(run_event);

            Python::with_gil(|py| {
                let callback = callback.bind(py);
                let result = callback.call1((py_app_handle.clone_ref(py), py_run_event));
                if let Err(e) = result {
                    // Use [write_unraisable] instead of [restore]:
                    // - Because we are about to panic, Python might abort
                    // - [restore] will not be handled in this case, so it will not be printed to stderr
                    e.write_unraisable(py, Some(callback));
                    // `panic` allows Python to exit `app.run()`,
                    // otherwise the Python main thread will be blocked by `app.run()`
                    // and unable to raise an error
                    panic!("Python exception occurred in callback")
                }
            })
        }
    }

    fn noop_callback(_: &tauri::AppHandle<Runtime>, _: tauri::RunEvent) {}
}

#[pymethods]
impl App {
    #[pyo3(signature = (callback = None, /))]
    fn run(&self, py: Python<'_>, callback: Option<PyObject>) -> PyResult<()> {
        // `self: &App` does not hold the GIL, so this is safe
        unsafe {
            py.allow_threads_unsend(self, |slf| {
                let app = slf.0.try_take_inner()??;
                match callback {
                    Some(callback) => app.run(Self::py_cb_to_rs_cb(callback)),
                    None => app.run(Self::noop_callback),
                }
                Ok(())
            })
        }
    }

    #[pyo3(signature = (callback = None, /))]
    fn run_iteration(&self, py: Python<'_>, callback: Option<PyObject>) -> PyResult<()> {
        unsafe {
            // `self: &App` does not hold the GIL, so this is safe
            py.allow_threads_unsend(self, |slf| {
                let mut app = slf.0.try_lock_inner_mut()??;
                match callback {
                    Some(callback) => app.run_iteration(Self::py_cb_to_rs_cb(callback)),
                    None => app.run_iteration(Self::noop_callback),
                }
                Ok(())
            })
        }
    }

    fn cleanup_before_exit(&self, py: Python<'_>) -> PyResult<()> {
        // `self: &App` does not hold the GIL, so this is safe
        unsafe {
            py.allow_threads_unsend(self, |slf| {
                let app = slf.0.try_lock_inner_ref()??;
                app.cleanup_before_exit();
                Ok(())
            })
        }
    }

    fn handle(&self, py: Python<'_>) -> PyResult<Py<AppHandle>> {
        let app = self.0.try_lock_inner_ref()??;
        let app_handle = app.py_app_handle().clone_ref(py);
        Ok(app_handle)
    }
}

#[pyclass(frozen)]
#[non_exhaustive]
pub struct Context(pub PyWrapper<PyWrapperT2<tauri::Context<Runtime>>>);

impl Context {
    pub fn new(context: tauri::Context<Runtime>) -> Self {
        Self(PyWrapper::new2(context))
    }
}

/// The Implementors of [tauri::Manager].
#[derive(FromPyObject, IntoPyObject, IntoPyObjectRef)]
#[non_exhaustive]
// TODO: more types
pub enum ImplManager {
    App(Py<App>),
    AppHandle(Py<AppHandle>),
    WebviewWindow(Py<webview::WebviewWindow>),
}

/// See also: [tauri::Manager].
#[pyclass(frozen)]
#[non_exhaustive]
pub struct Manager;

macro_rules! manager_method_impl {
    ($slf:expr, $macro:ident) => {
        match $slf {
            ImplManager::App(v) => $macro!(v),
            ImplManager::AppHandle(v) => $macro!(v),
            ImplManager::WebviewWindow(v) => $macro!(v),
        }
    };
}

// TODO, PERF: maybe we should release GIL?
// The methods of [Manager] seem require other locks internally.
#[pymethods]
impl Manager {
    #[staticmethod]
    fn app_handle(py: Python<'_>, slf: ImplManager) -> PyResult<Py<AppHandle>> {
        macro_rules! app_handle_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let app_handle = guard.py_app_handle().clone_ref(py);
                Ok(app_handle)
            }};
        }
        manager_method_impl!(slf, app_handle_impl)
    }

    #[staticmethod]
    fn get_webview_window(
        py: Python<'_>,
        slf: ImplManager,
        label: &str,
    ) -> PyResult<Option<webview::WebviewWindow>> {
        macro_rules! get_webview_window_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let webview_window = guard.get_webview_window(label);
                Ok(webview_window.map(webview::WebviewWindow::new))
            }};
        }
        manager_method_impl!(slf, get_webview_window_impl)
    }

    #[staticmethod]
    fn webview_windows(
        py: Python<'_>,
        slf: ImplManager,
    ) -> PyResult<HashMap<String, webview::WebviewWindow>> {
        macro_rules! webview_windows_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let webview_windows = guard.webview_windows();
                let webview_windows = webview_windows
                    .into_iter()
                    .map(|(label, window)| (label, webview::WebviewWindow::new(window)))
                    .collect::<_>();
                Ok(webview_windows)
            }};
        }
        manager_method_impl!(slf, webview_windows_impl)
    }
}

/// See also: [tauri::EventId].
pub use tauri::EventId;

/// See also: [tauri::Event].
#[pyclass(frozen)]
#[non_exhaustive]
pub struct Event {
    #[pyo3(get)]
    pub id: EventId,
    #[pyo3(get)]
    pub payload: Py<PyString>,
}

/// The Implementors of [tauri::Listener].
pub type ImplListener = ImplManager;

/// See also: [tauri::Listener].
#[pyclass(frozen)]
#[non_exhaustive]
pub struct Listener;

impl Listener {
    fn pyobj_to_handler(pyobj: PyObject) -> impl Fn(tauri::Event) + Send + 'static {
        move |event| {
            Python::with_gil(|py| {
                let event = Event {
                    id: event.id(),
                    payload: PyString::new(py, event.payload()).unbind(),
                };
                let pyobj = pyobj.bind(py);
                let result = pyobj.call1((event,));
                if let Err(e) = result {
                    e.write_unraisable(py, Some(pyobj));
                    panic!("Python exception occurred in Listener handler")
                }
            })
        }
    }
}

// TODO, PERF: maybe we should release GIL?
// The methods of [Listener] seem require other locks internally.
#[pymethods]
impl Listener {
    #[staticmethod]
    fn listen(
        py: Python<'_>,
        slf: ImplListener,
        event: Cow<'_, str>,
        handler: PyObject,
    ) -> PyResult<EventId> {
        macro_rules! listen_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let enevt_id = guard.listen(event, Self::pyobj_to_handler(handler));
                Ok(enevt_id)
            }};
        }
        manager_method_impl!(slf, listen_impl)
    }

    #[staticmethod]
    fn once(
        py: Python<'_>,
        slf: ImplListener,
        event: Cow<'_, str>,
        handler: PyObject,
    ) -> PyResult<EventId> {
        macro_rules! once_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let enevt_id = guard.once(event, Self::pyobj_to_handler(handler));
                Ok(enevt_id)
            }};
        }
        manager_method_impl!(slf, once_impl)
    }

    #[staticmethod]
    fn unlisten(py: Python<'_>, slf: ImplListener, id: EventId) -> PyResult<()> {
        macro_rules! unlisten_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                guard.unlisten(id);
                Ok(())
            }};
        }
        manager_method_impl!(slf, unlisten_impl)
    }

    #[staticmethod]
    fn listen_any(
        py: Python<'_>,
        slf: ImplListener,
        event: Cow<'_, str>,
        handler: PyObject,
    ) -> PyResult<EventId> {
        macro_rules! listen_any_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let enevt_id = guard.listen_any(event, Self::pyobj_to_handler(handler));
                Ok(enevt_id)
            }};
        }
        manager_method_impl!(slf, listen_any_impl)
    }

    #[staticmethod]
    fn once_any(
        py: Python<'_>,
        slf: ImplListener,
        event: Cow<'_, str>,
        handler: PyObject,
    ) -> PyResult<EventId> {
        macro_rules! once_any_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let enevt_id = guard.once_any(event, Self::pyobj_to_handler(handler));
                Ok(enevt_id)
            }};
        }
        manager_method_impl!(slf, once_any_impl)
    }
}

/// See also: [tauri::EventTarget].
#[pyclass(frozen)]
#[non_exhaustive]
pub enum EventTarget {
    Any(),
    AnyLabel { label: Py<PyString> },
    App(),
    Window { label: Py<PyString> },
    Webview { label: Py<PyString> },
    WebviewWindow { label: Py<PyString> },
}

impl EventTarget {
    fn from_tauri(value: &tauri::EventTarget, py: Python<'_>) -> Self {
        match value {
            tauri::EventTarget::Any => Self::Any(),
            tauri::EventTarget::AnyLabel { label } => Self::AnyLabel {
                label: PyString::new(py, label).unbind(),
            },
            tauri::EventTarget::App => Self::App(),
            tauri::EventTarget::Window { label } => Self::Window {
                label: PyString::new(py, label).unbind(),
            },
            tauri::EventTarget::Webview { label } => Self::Webview {
                label: PyString::new(py, label).unbind(),
            },
            tauri::EventTarget::WebviewWindow { label } => Self::WebviewWindow {
                label: PyString::new(py, label).unbind(),
            },
            target => {
                unimplemented!("Please make a issue for unimplemented EventTarget: {target:?}")
            }
        }
    }

    fn to_tauri(&self, py: Python<'_>) -> PyResult<tauri::EventTarget> {
        // TODO, PERF: once we drop py39, we can use [PyStringMethods::to_str] instead of [PyStringMethods::to_cow]
        let value = match self {
            Self::Any() => tauri::EventTarget::Any,
            Self::AnyLabel { label } => tauri::EventTarget::AnyLabel {
                label: label.bind(py).to_cow()?.into_owned(),
            },
            Self::App() => tauri::EventTarget::App,
            Self::Window { label } => tauri::EventTarget::Window {
                label: label.bind(py).to_cow()?.into_owned(),
            },
            Self::Webview { label } => tauri::EventTarget::Webview {
                label: label.bind(py).to_cow()?.into_owned(),
            },
            Self::WebviewWindow { label } => tauri::EventTarget::WebviewWindow {
                label: label.bind(py).to_cow()?.into_owned(),
            },
        };
        Ok(value)
    }
}

/// The Implementors of [tauri::Emitter].
pub type ImplEmitter = ImplManager;

/// See also: [tauri::Emitter].
#[pyclass(frozen, subclass)]
#[non_exhaustive]
pub struct Emitter;

impl Emitter {
    fn assert_is_emitter<T: tauri::Emitter<Runtime>>(v: &T) -> &T {
        v
    }
}

#[pymethods]
impl Emitter {
    #[staticmethod]
    fn emit_str(py: Python<'_>, slf: ImplEmitter, event: &str, payload: String) -> PyResult<()> {
        macro_rules! emit_str_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let guard = Self::assert_is_emitter(guard.deref());
                // NOTE: we had better release the GIL, because `emit` is not a very fast operation,
                // it even acquires other locks internally.
                //
                // Safe: `guard: &impl tauri::Emitter` does not hold the GIL, so this is safe.
                unsafe {
                    py.allow_threads_unsend(guard, |guard| {
                        guard.emit_str(event, payload).map_err(TauriError::from)
                    })?;
                }
                Ok(())
            }};
        }
        manager_method_impl!(slf, emit_str_impl)
    }

    #[staticmethod]
    fn emit_str_to(
        py: Python<'_>,
        slf: ImplEmitter,
        target: Py<EventTarget>,
        event: &str,
        payload: String,
    ) -> PyResult<()> {
        let target = target.get().to_tauri(py)?;

        macro_rules! emit_str_to_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let guard = Self::assert_is_emitter(guard.deref());
                // NOTE: we had better release the GIL, because `emit` is not a very fast operation,
                // it even acquires other locks internally.
                //
                // Safe: `guard: &impl tauri::Emitter` does not hold the GIL, so this is safe.
                unsafe {
                    py.allow_threads_unsend(guard, |guard| {
                        guard
                            .emit_str_to(target, event, payload)
                            .map_err(TauriError::from)
                    })?;
                }
                Ok(())
            }};
        }
        manager_method_impl!(slf, emit_str_to_impl)
    }

    #[staticmethod]
    fn emit_str_filter(
        py: Python<'_>,
        slf: ImplEmitter,
        event: &str,
        payload: String,
        filter: Bound<PyAny>,
    ) -> PyResult<()> {
        fn unwrap_unraisable_py_result<T>(
            py: Python<'_>,
            result: PyResult<T>,
            obj: Option<&Bound<'_, PyAny>>,
            panic: &'static str,
        ) -> T {
            match result {
                Ok(value) => value,
                Err(err) => {
                    err.write_unraisable(py, obj);
                    std::panic::panic_any(panic)
                }
            }
        }

        // We can't release the GIL here, because `rs_filter` will be used as `iter.filter(|..| rs_filter(..))`;
        // if we frequently release and acquire the GIL, maybe it will cause performance problems.
        // TODO: only tauri itself can release GIL in `emit_str_filter`.
        let rs_filter = |target: &tauri::EventTarget| -> bool {
            let target = EventTarget::from_tauri(target, py);
            let filter_result = filter.call1((target,));
            let filter_return = unwrap_unraisable_py_result(
                py,
                filter_result,
                Some(&filter),
                "Python exception occurred in emitter filter",
            );
            let extract_result = filter_return.extract::<bool>();

            unwrap_unraisable_py_result(
                py,
                extract_result,
                Some(&filter_return),
                "emitter filter return non-bool value",
            )
        };

        macro_rules! emit_str_filter_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                guard
                    .emit_str_filter(event, payload, rs_filter)
                    .map_err(TauriError::from)?;
                Ok(())
            }};
        }
        manager_method_impl!(slf, emit_str_filter_impl)
    }
}
