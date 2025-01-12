// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env::var;

use pyo3::{prelude::*, py_run, types::PyDict, wrap_pymodule};
use pytauri::standalone::{
    append_ext_mod, get_python_executable_from_venv, prepare_freethreaded_python_with_executable,
    write_py_err_to_file,
};
use tauri::{Builder, Manager as _};

use nicegui_app_lib::{ext_mod, tauri_generate_context};

fn prepare_python_interpreter() {
    // `cfg(dev)` is set by `tauri-build` in `build.rs`, which means running with `tauri dev`,
    // see: <https://github.com/tauri-apps/tauri/pull/8937>.
    if cfg!(dev) {
        // virtualenv Python
        //
        // See:
        //
        // - <https://github.com/PyO3/pyo3/issues/3589>
        // - <https://github.com/PyO3/pyo3/issues/1896>
        //
        // pyo3 currently cannot automatically detect the virtual environment and configure pyconfig,
        // so we do it manually here.
        let venv_path = var("VIRTUAL_ENV").expect(
            "The app is running in tauri dev mode, \
                please activate the python virtual environment first \
                or set the `VIRTUAL_ENV` environment variable",
        );
        let python_executable = get_python_executable_from_venv(venv_path);
        prepare_freethreaded_python_with_executable(python_executable)
            .expect("failed to initialize python from venv");
    } else {
        // embedded Python, i.e., bundle mode with `tauri build`.

        // Actually, we don't use this app, we just use it to get the resource directory
        let sample_app = Builder::default()
            .build(tauri_generate_context())
            .expect("failed to build sample app");
        let resource_dir = sample_app
            .path()
            .resource_dir()
            .expect("failed to get resource dir");

        let py_executable = if cfg!(windows) {
            resource_dir.join("python.exe")
        } else {
            resource_dir.join("bin/python3")
        };

        debug_assert!(
            py_executable.is_file(),
            "Python executable not found, maybe you forgot to bundle it: {}",
            py_executable.display()
        );

        prepare_freethreaded_python_with_executable(py_executable)
            .expect("failed to initialize embedded python");
    }
}

fn execute_python_script(py: Python<'_>) -> PyResult<()> {
    // insert pytauri extension module into `sys.modules`
    append_ext_mod(wrap_pymodule!(ext_mod)(py).into_bound(py))?;

    // execute your Python script
    py.run(
        // equivalent to `python -m nicegui_app`
        c"from runpy import run_module; run_module('nicegui_app')",
        None,
        None,
    )
}

fn main() -> Result<(), PyErr> {
    prepare_python_interpreter();

    Python::with_gil(|py| {
        // This is an env var that can only be used internally by pytauri to distinguish
        // between different example extension modules.
        // You don't need and shouldn't set this in your own app.
        // Must be set before calling [append_ext_mod]
        py_run!(
            py,
            *PyDict::new(py),
            r#"
            from os import environ
            environ["_PYTAURI_DIST"] = "nicegui-app"
            "#
        );

        let result = execute_python_script(py);

        // handle the error
        result.inspect_err(|e| {
            if cfg!(all(not(debug_assertions), windows)) {
                // I.g., `windows_subsystem = "windows"` in `main.rs`.
                // In this case, there is no console to print the error, so we write the error to a file
                write_py_err_to_file(py, e, "error.log").expect("failed to write error to file");
            } else {
                // we have a console, so we just call `sys.excepthook` and print the error
                e.print_and_set_sys_last_vars(py);
            }
        })
    })
}
