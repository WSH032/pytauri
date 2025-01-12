use pyo3::prelude::*;
use pyo3_utils::py_wrapper::{PyWrapper, PyWrapperT0};
use tauri::webview;

use crate::tauri_runtime::Runtime;
use crate::utils::TauriError;

type TauriWebviewWindow = webview::WebviewWindow<Runtime>;

#[pyclass(frozen)]
#[non_exhaustive]
pub struct WebviewWindow(pub PyWrapper<PyWrapperT0<TauriWebviewWindow>>);

impl WebviewWindow {
    pub(crate) fn new(window: TauriWebviewWindow) -> Self {
        Self(PyWrapper::new0(window))
    }
}

#[pymethods]
impl WebviewWindow {
    fn hide(&self) -> PyResult<()> {
        self.0.inner_ref().hide().map_err(TauriError::from)?;
        Ok(())
    }

    fn show(&self) -> PyResult<()> {
        self.0.inner_ref().show().map_err(TauriError::from)?;
        Ok(())
    }

    fn eval(&self, js: &str) -> PyResult<()> {
        self.0.inner_ref().eval(js).map_err(TauriError::from)?;
        Ok(())
    }
}
