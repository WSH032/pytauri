use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyString};
use pyo3_utils::py_wrapper::{PyWrapper, PyWrapperT0};
use tauri::{webview, Url};

use crate::tauri_runtime::Runtime;
use crate::utils::TauriError;

type TauriWebviewWindow = webview::WebviewWindow<Runtime>;
type TauriWebview = webview::Webview<Runtime>;

#[pyclass(frozen)]
#[non_exhaustive]
pub struct WebviewWindow(pub PyWrapper<PyWrapperT0<TauriWebviewWindow>>);

impl WebviewWindow {
    pub(crate) fn new(webview_window: TauriWebviewWindow) -> Self {
        Self(PyWrapper::new0(webview_window))
    }
}

macro_rules! delegate_inner {
    ($slf:expr, $func:ident, $($arg:expr),*) => {
        $slf.0
            .inner_ref()
            .$func($($arg),*)
            .map_err(|e| PyErr::from(TauriError::from(e)))
    };
}

#[pymethods]
impl WebviewWindow {
    fn label<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        let webview_window = self.0.inner_ref();
        PyString::new(py, webview_window.label())
    }

    fn is_fullscreen(&self) -> PyResult<bool> {
        delegate_inner!(self, is_fullscreen,)
    }

    fn is_minimized(&self) -> PyResult<bool> {
        delegate_inner!(self, is_minimized,)
    }

    fn is_maximized(&self) -> PyResult<bool> {
        delegate_inner!(self, is_maximized,)
    }

    fn is_focused(&self) -> PyResult<bool> {
        delegate_inner!(self, is_focused,)
    }

    fn is_decorated(&self) -> PyResult<bool> {
        delegate_inner!(self, is_decorated,)
    }

    fn is_resizable(&self) -> PyResult<bool> {
        delegate_inner!(self, is_resizable,)
    }

    fn is_enabled(&self) -> PyResult<bool> {
        delegate_inner!(self, is_enabled,)
    }

    fn is_maximizable(&self) -> PyResult<bool> {
        delegate_inner!(self, is_maximizable,)
    }

    fn is_minimizable(&self) -> PyResult<bool> {
        delegate_inner!(self, is_minimizable,)
    }

    fn is_closable(&self) -> PyResult<bool> {
        delegate_inner!(self, is_closable,)
    }

    fn is_visible(&self) -> PyResult<bool> {
        delegate_inner!(self, is_visible,)
    }

    fn title(&self) -> PyResult<String> {
        delegate_inner!(self, title,)
    }

    fn center(&self) -> PyResult<()> {
        delegate_inner!(self, center,)
    }

    fn set_resizable(&self, resizable: bool) -> PyResult<()> {
        delegate_inner!(self, set_resizable, resizable)
    }

    fn set_enabled(&self, enabled: bool) -> PyResult<()> {
        delegate_inner!(self, set_enabled, enabled)
    }

    fn set_maximizable(&self, maximizable: bool) -> PyResult<()> {
        delegate_inner!(self, set_maximizable, maximizable)
    }

    fn set_minimizable(&self, minimizable: bool) -> PyResult<()> {
        delegate_inner!(self, set_minimizable, minimizable)
    }

    fn set_closable(&self, closable: bool) -> PyResult<()> {
        delegate_inner!(self, set_closable, closable)
    }

    fn set_title(&self, title: &str) -> PyResult<()> {
        delegate_inner!(self, set_title, title)
    }

    fn maximize(&self) -> PyResult<()> {
        delegate_inner!(self, maximize,)
    }

    fn unmaximize(&self) -> PyResult<()> {
        delegate_inner!(self, unmaximize,)
    }

    fn minimize(&self) -> PyResult<()> {
        delegate_inner!(self, minimize,)
    }

    fn unminimize(&self) -> PyResult<()> {
        delegate_inner!(self, unminimize,)
    }

    fn show(&self) -> PyResult<()> {
        delegate_inner!(self, show,)
    }

    fn hide(&self) -> PyResult<()> {
        delegate_inner!(self, hide,)
    }

    fn close(&self) -> PyResult<()> {
        delegate_inner!(self, close,)
    }

    fn destroy(&self) -> PyResult<()> {
        delegate_inner!(self, destroy,)
    }

    fn set_decorations(&self, decorations: bool) -> PyResult<()> {
        delegate_inner!(self, set_decorations, decorations)
    }

    fn set_shadow(&self, shadow: bool) -> PyResult<()> {
        delegate_inner!(self, set_shadow, shadow)
    }

    fn set_always_on_bottom(&self, always_on_bottom: bool) -> PyResult<()> {
        delegate_inner!(self, set_always_on_bottom, always_on_bottom)
    }

    fn set_always_on_top(&self, always_on_top: bool) -> PyResult<()> {
        delegate_inner!(self, set_always_on_top, always_on_top)
    }

    fn set_visible_on_all_workspaces(&self, visible_on_all_workspaces: bool) -> PyResult<()> {
        delegate_inner!(
            self,
            set_visible_on_all_workspaces,
            visible_on_all_workspaces
        )
    }

    fn set_content_protected(&self, protected: bool) -> PyResult<()> {
        delegate_inner!(self, set_content_protected, protected)
    }

    fn set_fullscreen(&self, fullscreen: bool) -> PyResult<()> {
        delegate_inner!(self, set_fullscreen, fullscreen)
    }

    fn set_focus(&self) -> PyResult<()> {
        delegate_inner!(self, set_focus,)
    }

    fn set_skip_taskbar(&self, skip: bool) -> PyResult<()> {
        delegate_inner!(self, set_skip_taskbar, skip)
    }

    fn set_cursor_grab(&self, grab: bool) -> PyResult<()> {
        delegate_inner!(self, set_cursor_grab, grab)
    }

    fn set_cursor_visible(&self, visible: bool) -> PyResult<()> {
        delegate_inner!(self, set_cursor_visible, visible)
    }

    fn set_ignore_cursor_events(&self, ignore: bool) -> PyResult<()> {
        delegate_inner!(self, set_ignore_cursor_events, ignore)
    }

    fn start_dragging(&self) -> PyResult<()> {
        delegate_inner!(self, start_dragging,)
    }

    #[pyo3(signature = (count))]
    fn set_badge_count(&self, count: Option<i64>) -> PyResult<()> {
        delegate_inner!(self, set_badge_count, count)
    }

    fn print(&self) -> PyResult<()> {
        delegate_inner!(self, print,)
    }

    fn url<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyString>> {
        let url = delegate_inner!(self, url,)?;
        Ok(PyString::new(py, url.as_ref()))
    }

    fn navigate(&self, url: &str) -> PyResult<()> {
        let url = Url::parse(url).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        delegate_inner!(self, navigate, url)
    }

    fn eval(&self, js: &str) -> PyResult<()> {
        delegate_inner!(self, eval, js)
    }

    fn set_zoom(&self, scale_factor: f64) -> PyResult<()> {
        delegate_inner!(self, set_zoom, scale_factor)
    }

    fn clear_all_browsing_data(&self) -> PyResult<()> {
        delegate_inner!(self, clear_all_browsing_data,)
    }

    /// see also: [tauri::webview::WebviewWindow::as_ref]
    fn as_ref_webview(&self) -> Webview {
        let webview = self.0.inner_ref().as_ref().clone();
        Webview::new(webview)
    }
}

/// see also: [tauri::webview::Webview]
#[pyclass(frozen)]
#[non_exhaustive]
pub struct Webview(pub PyWrapper<PyWrapperT0<TauriWebview>>);

impl Webview {
    pub(crate) fn new(webview: TauriWebview) -> Self {
        Self(PyWrapper::new0(webview))
    }
}
