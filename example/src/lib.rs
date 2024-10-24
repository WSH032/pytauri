use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "_ext_mod")]
mod _ext_mod {
    use super::*;

    #[pymodule_export]
    use pytauri_plugin_notification::notification;

    #[pymodule_init]
    fn init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        tauri_plugin_pytauri::pymodule_export(
            module,
            |_kwargs| {
                let builder = tauri::Builder::default()
                    .plugin(tauri_plugin_shell::init())
                    .plugin(tauri_plugin_pytauri::init())
                    .plugin(tauri_plugin_notification::init());
                Ok(builder)
            },
            || Ok(tauri::generate_context!()),
        )
    }
}
