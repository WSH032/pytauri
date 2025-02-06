use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3_utils::py_wrapper::{PyWrapper, PyWrapperT0};
use tauri::path;

use crate::{tauri_runtime::Runtime, utils::TauriError};

type TauriPathResolver = path::PathResolver<Runtime>;

/// see also: [tauri::path::PathResolver].
#[pyclass(frozen)]
#[non_exhaustive]
pub struct PathResolver(PyWrapper<PyWrapperT0<TauriPathResolver>>);

impl PathResolver {
    pub(crate) fn new(path_resolver: TauriPathResolver) -> Self {
        Self(PyWrapper::new0(path_resolver))
    }
}

macro_rules! impl_path_resolver_method {
    ($($fn_name:ident),*) => {
        #[pymethods]
        impl PathResolver {
            $(
                fn $fn_name(&self) -> PyResult<PathBuf> {
                    self.0
                        .inner_ref()
                        .$fn_name()
                        .map_err(|e| PyErr::from(TauriError::from(e)))
                }
            )*
        }
    };

}

impl_path_resolver_method!(
    audio_dir,
    cache_dir,
    config_dir,
    data_dir,
    local_data_dir,
    desktop_dir,
    document_dir,
    download_dir,
    executable_dir,
    font_dir,
    home_dir,
    picture_dir,
    public_dir,
    runtime_dir,
    template_dir,
    video_dir,
    resource_dir,
    app_config_dir,
    app_data_dir,
    app_local_data_dir,
    app_cache_dir,
    app_log_dir,
    temp_dir
);
