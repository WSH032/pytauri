use std::error::Error;
use std::fmt::{Debug, Display};

use pyo3::prelude::*;
use pyo3_utils::py_wrapper::{PyWrapper, PyWrapperSemverExt as _, PyWrapperT2};
use pytauri_core::{ext_mod::ImplManager, tauri_runtime::Runtime};
use tauri_plugin_notification::{self as plugin, NotificationExt as _};

#[derive(Debug)]
struct PluginError(plugin::Error);

impl Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl Error for PluginError {}

impl From<PluginError> for PyErr {
    fn from(value: PluginError) -> Self {
        match value.0 {
            plugin::Error::Io(e) => e.into(),
        }
    }
}

impl From<plugin::Error> for PluginError {
    fn from(value: plugin::Error) -> Self {
        Self(value)
    }
}

// We use a `newtype` instead of directly implementing methods like `show(title=...)`,
// This is to facilitate the addition of new methods in the future,
// such as `hide(NotificationBuilderArgs)` instead of repeatedly declaring `hide(title=...)`
#[pyclass(frozen)]
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct NotificationBuilderArgs {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[pymethods]
impl NotificationBuilderArgs {
    #[new]
    #[pyo3(signature = (*, title = None, body = None))]
    fn new(title: Option<String>, body: Option<String>) -> Self {
        Self { title, body }
    }
}

impl NotificationBuilderArgs {
    fn call_show(self, mut builder: plugin::NotificationBuilder<Runtime>) -> plugin::Result<()> {
        let Self { title, body } = self;
        if let Some(v) = title {
            builder = builder.title(v)
        }
        if let Some(v) = body {
            builder = builder.body(v)
        }
        builder.show()
    }
}

#[pyclass(frozen)]
#[non_exhaustive]
pub struct NotificationBuilder(pub PyWrapper<PyWrapperT2<plugin::NotificationBuilder<Runtime>>>);

impl NotificationBuilder {
    fn new(builder: plugin::NotificationBuilder<Runtime>) -> Self {
        Self(PyWrapper::new2(builder))
    }
}

#[pymethods]
impl NotificationBuilder {
    fn show(&self, py: Python<'_>, args: NotificationBuilderArgs) -> PyResult<()> {
        // TODO (perf): Do we really need `py.allow_threads` here?
        // I mean, I don't know how long `NotificationBuilder::show` will take,
        // maybe it's short enough?
        py.allow_threads(|| {
            let builder = self.0.try_take_inner()??;
            args.call_show(builder)
                .map_err(Into::<PluginError>::into)
                .map_err(Into::<PyErr>::into)
        })
    }
}

#[pyclass(frozen)]
#[non_exhaustive]
pub struct NotificationExt;

pub type ImplNotificationExt = ImplManager;

#[pymethods]
impl NotificationExt {
    // TODO: Add `struct Notification` as an intermediate layer, currently blocked by:
    // <https://github.com/tauri-apps/plugins-workspace/issues/2161>

    #[staticmethod]
    fn builder(slf: ImplNotificationExt, py: Python<'_>) -> PyResult<NotificationBuilder> {
        macro_rules! builder_impl {
            ($wrapper:expr) => {{
                let py_ref = $wrapper.borrow(py);
                let guard = py_ref.0.inner_ref_semver()??;
                let builder = guard.notification().builder();
                Ok(NotificationBuilder::new(builder))
            }};
        }
        match slf {
            ImplNotificationExt::App(v) => builder_impl!(v),
            ImplNotificationExt::AppHandle(v) => builder_impl!(v),
            _ => unimplemented!("please create an feature request to pytauri"),
        }
    }
}
