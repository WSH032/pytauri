// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env::var;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pytauri::standalone::{
    append_ext_mod, get_python_executable_from_venv, prepare_freethreaded_python_with_executable,
};

use _ext_mod::_ext_mod;

fn main() -> Result<(), PyErr> {
    if let Ok(venv_path) = var("VIRTUAL_ENV") {
        // See:
        //
        // - <https://github.com/PyO3/pyo3/issues/3589>
        // - <https://github.com/PyO3/pyo3/issues/1896>
        //
        // pyo3 currently cannot automatically detect the virtual environment and configure pyconfig,
        // so we do it manually here.
        let python_executable = get_python_executable_from_venv(venv_path);
        prepare_freethreaded_python_with_executable(python_executable)
            .expect("failed to initialize python from venv");
    } else {
        #[cfg(windows)]
        {
            use std::path::absolute;
            // The embedded Python and the pytauri app are in the same directory
            let python_executable = absolute("python.exe").unwrap();
            prepare_freethreaded_python_with_executable(python_executable)
                .expect("failed to initialize embedded python");
        }
        #[cfg(target_os = "linux")]
        {
            use std::path::PathBuf;
            // NOTE: set it to your `productName` in `Tauri.toml`
            let product_name = "pytauri-demo";
            // The resource directory specified by tauri-cli when packaging in Debian.
            // But this is a bit hacky because this is not the documented behavior of tauri-cli,
            // we need to contact the tauri developers.
            // See also: <https://github.com/WSH032/pytauri/commit/b0c754aed9e7095c0a994978cbe191cf56022158#diff-8aa8e83ff3b2189fd6397149a7b274f82daede3923191a3d0409f72a2f8bb78fR39>
            let resource_dir: PathBuf = format!("/usr/lib/{}/", product_name).into();
            // the bundled Python executable in the resource directory
            let python_executable = resource_dir.join("bin/python3");
            prepare_freethreaded_python_with_executable(python_executable)
                .expect("failed to initialize embedded python");
        }
        #[cfg(target_os = "macos")]
        {
            todo!("Support for other platforms is still being implemented");
        }
    }

    Python::with_gil(|py| {
        let script = || {
            append_ext_mod(wrap_pymodule!(_ext_mod)(py).into_bound(py))?;

            // Run your python code here
            Python::run(
                py,
                // equal to `python -m pytauri_demo`
                c"from runpy import run_module; run_module('pytauri_demo')",
                None,
                None,
            )?;

            Ok::<_, PyErr>(())
        };

        script().inspect_err(|e| {
            #[cfg(all(not(debug_assertions), windows))]
            {
                // In this case, there is no console to print the error, so we write the error to a file
                use pytauri::standalone::write_py_err_to_file;
                write_py_err_to_file(py, &e, "error.log").expect("failed to write error to file");
            }
            #[cfg(not(all(not(debug_assertions), windows)))]
            {
                e.print_and_set_sys_last_vars(py);
            }
        })
    })
}
