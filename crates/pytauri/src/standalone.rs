//! Used for embedding Python in a standalone pytauri application.
//!
//! In most cases you will use:
//!
//! - [prepare_freethreaded_python_with_executable]
//! - [append_ext_mod]
//!
//! # NOTE
//!
//! This module is licensed under the `MPL-2.0` license from [pyembed].
//!
//! [pyembed]: https://crates.io/crates/pyembed

use std::{
    env::{args_os, current_exe},
    ffi::OsString,
    fs, io,
    path::{absolute, Path, PathBuf},
};

use pyo3::{
    ffi::{self as pyffi, c_str},
    prelude::*,
    prepare_freethreaded_python,
    types::PyModule,
};

use crate::pyembed::{utils, NewInterpreterError, NewInterpreterResult};

#[non_exhaustive]
pub enum PyConfigProfile {
    Python,
    Isolated,
}

/// see: <https://docs.python.org/3/c-api/init_config.html#c.PyConfig>
pub struct PyConfig(pyffi::PyConfig);

/// NOTE: You can use [PyConfig] only before calling [pyo3::prepare_freethreaded_python],
/// otherwise it is an invalid operation
// ref: <https://github.com/indygreg/PyOxidizer/blob/1ceca8664c71f39e849ce4873e00d821504b32bd/pyembed/src/interpreter_config.rs#L252-L619>
impl PyConfig {
    pub fn new(profile: PyConfigProfile) -> Self {
        let mut config: pyffi::PyConfig = unsafe { std::mem::zeroed() };

        unsafe {
            match profile {
                PyConfigProfile::Isolated => pyffi::PyConfig_InitIsolatedConfig(&mut config),
                PyConfigProfile::Python => pyffi::PyConfig_InitPythonConfig(&mut config),
            }
        }

        Self(config)
    }

    pub fn set_prefix(&mut self, prefix: &Path) -> NewInterpreterResult<()> {
        utils::set_config_string_from_path(&self.0, &self.0.prefix, prefix, "setting prefix")
    }

    pub fn set_base_prefix(&mut self, base_prefix: &Path) -> NewInterpreterResult<()> {
        utils::set_config_string_from_path(
            &self.0,
            &self.0.base_prefix,
            base_prefix,
            "setting base_prefix",
        )
    }

    pub fn set_exec_prefix(&mut self, exec_prefix: &Path) -> NewInterpreterResult<()> {
        utils::set_config_string_from_path(
            &self.0,
            &self.0.exec_prefix,
            exec_prefix,
            "setting exec_prefix",
        )
    }

    pub fn set_base_exec_prefix(&mut self, base_exec_prefix: &Path) -> NewInterpreterResult<()> {
        utils::set_config_string_from_path(
            &self.0,
            &self.0.base_exec_prefix,
            base_exec_prefix,
            "setting base_exec_prefix",
        )
    }

    pub fn set_program_name(&mut self, program_name: &Path) -> NewInterpreterResult<()> {
        utils::set_config_string_from_path(
            &self.0,
            &self.0.program_name,
            program_name,
            "setting program_name",
        )
    }

    pub fn set_executable(&mut self, executable: &Path) -> NewInterpreterResult<()> {
        utils::set_config_string_from_path(
            &self.0,
            &self.0.executable,
            executable,
            "setting executable",
        )
    }

    pub fn set_argv(&mut self, args: &[OsString]) -> NewInterpreterResult<()> {
        utils::set_argv(&mut self.0, args)
    }

    pub fn set_parse_argv(&mut self, parse_argv: bool) {
        self.0.parse_argv = if parse_argv { 1 } else { 0 };
    }

    /// After calling this method, the Python interpreter has already been initialized,
    /// so [pyo3::prepare_freethreaded_python] is a no-op.
    /// But you still need to call it to let pyo3 know that the interpreter has been initialized.
    // ref: <https://github.com/indygreg/PyOxidizer/blob/1ceca8664c71f39e849ce4873e00d821504b32bd/pyembed/src/interpreter.rs#L130-L255>
    pub fn init(self) -> NewInterpreterResult<()> {
        let status = unsafe { pyffi::Py_InitializeFromConfig(&self.0) };
        if unsafe { pyffi::PyStatus_Exception(status) } != 0 {
            return Err(NewInterpreterError::new_from_pystatus(
                &status,
                "initializing Python core",
            ));
        }
        // The GIL is held.
        debug_assert_eq!(unsafe { pyffi::PyGILState_Check() }, 1);

        // We release the GIL so we can have pyo3's GIL handling take over from
        // an "empty" state. This mirrors what pyo3's prepare_freethreaded_python() does.
        unsafe {
            pyffi::PyEval_SaveThread();
        }
        Ok(())
    }
}

impl Drop for PyConfig {
    fn drop(&mut self) {
        unsafe {
            pyffi::PyConfig_Clear(&mut self.0);
        }
    }
}

/// Get the absolute path of the Python interpreter from the virtual environment path.
///
/// For Unix systems, it is equivalent to `venv_path / "bin/python3"`.
///
/// For Windows systems, it is equivalent to `venv_path / "Scripts/python.exe"`.
///
/// # Panics
///
/// Panics if [std::path::absolute] fails.
pub fn get_python_executable_from_venv(venv_path: impl Into<PathBuf>) -> PathBuf {
    let mut venv_path: PathBuf = venv_path.into();
    #[cfg(unix)]
    venv_path.push("bin/python3");
    #[cfg(windows)]
    venv_path.push(r"Scripts\python.exe");
    #[cfg(not(any(unix, windows)))]
    unimplemented!();
    // NOTE: Use [std::path::absolute] instead of [Path::canonicalize].
    // On Unix, the Python executable in the virtual environment is actually a symbolic link,
    // [canonicalize] will resolve the symbolic link,
    // causing the path to be set to the system-level Python interpreter.
    absolute(&venv_path).expect("failed to get absolute path")
}

/// Whether the Python interpreter is in "multiprocessing worker" mode.
///
/// The `multiprocessing` module can work by spawning new processes
/// with arguments `--multiprocessing-fork [key=value] ...`. This function
/// detects if the current Python interpreter is configured for said execution.
///
/// Useful if you want to use cil arg parsing lib like `clap` in your standalone app.
///
/// # NOTE
///
/// This function only work on Windows with `spawn` multiprocessing start method.
/// On Unix, you should use `fork` start method.
// ---
//
// ref: <https://github.com/indygreg/PyOxidizer/blob/ae36f8672d905a911f1b8243308fe45c5fe981de/pyembed/src/interpreter.rs#L582-L591>
#[cfg(any(windows, doc))]
pub fn is_forking() -> bool {
    let mut argv = args_os();
    if let Some(arg) = argv.nth(1) {
        arg == "--multiprocessing-fork"
    } else {
        false
    }
}

fn freeze(py: Python<'_>) -> PyResult<()> {
    let current_exe = current_exe()?;

    let _freeze = PyModule::from_code(
        py,
        c_str!(include_str!("_freeze.py")),
        c"_freeze.py",
        c"_freeze",
    )?;

    _freeze.getattr("freeze")?.call1((current_exe,))?;

    Ok(())
}

/// Prepare a Python interpreter with the specified Python executable.
///
/// See:
///
/// - <https://docs.python.org/3/c-api/intro.html#embedding-python>
/// - <https://docs.python.org/3/c-api/init_config.html#python-path-configuration>
///
/// > The embedding application can steer the search by setting PyConfig.program_name before calling Py_InitializeFromConfig().
///
/// Once you set `program_name` and `executable` to the actual Python executable,
/// the Python interpreter will automatically set other paths config, such as `sys.prefix`, `sys.path`,
/// then the std lib and site-packages will be found correctly.
///
/// NOTE: the `executable` must be absolute path.
///
/// This method will internally call [pyo3::prepare_freethreaded_python], so there is no need to call it manually.
///
/// # Example
/**
```no_run
use pyo3::prelude::*;
use pytauri::standalone::prepare_freethreaded_python_with_executable;

prepare_freethreaded_python_with_executable("/my/python")
    .expect("failed to prepare python interpreter");
Python::with_gil(|_py| {
    // Your code here
});
```
*/
pub fn prepare_freethreaded_python_with_executable(
    executable: impl AsRef<Path>,
) -> NewInterpreterResult<()> {
    let executable = executable.as_ref();

    let mut config = PyConfig::new(PyConfigProfile::Python);
    config.set_program_name(executable)?;
    config.set_executable(executable)?;
    config.set_argv(&args_os().collect::<Vec<_>>())?;
    // `parse_argv=false` is necessary, because python only accepts following argv pattern:
    //
    // ```shell
    // # <https://docs.python.org/3/using/cmdline.html#using-on-cmdline>
    // python [-bBdEhiIOPqRsSuvVWx?] [-c command | -m module-name | script | - ] [args]
    // ```
    //
    // This will prevent us from using libraries like `clap` to parse command line arguments
    config.set_parse_argv(false);
    config.init()?;

    prepare_freethreaded_python();

    Python::with_gil(|py| {
        freeze(py).map_err(|e| {
            NewInterpreterError::new_from_pyerr(py, e, "failed to freeze the Python interpreter")
        })
    })?;

    Ok(())
}

/// Insert the `pytauri` extension module into `sys.modules`,
/// otherwise your Python code cannot import `pytauri` when built as a standalone application.
///
/// # Example
/**
```no_run
use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pytauri::standalone::append_ext_mod;

// Your extension module
#[pymodule]
mod ext_mod {}

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let ext_mod = wrap_pymodule!(ext_mod)(py).into_bound(py);
        append_ext_mod(ext_mod)?;
        // Your code here
        Ok(())
    })
}
```
*/
pub fn append_ext_mod(ext_mod: Bound<PyModule>) -> PyResult<()> {
    let py = ext_mod.py();

    let _append_ext_mod = PyModule::from_code(
        py,
        c_str!(include_str!("_append_ext_mod.py")),
        c"_append_ext_mod.py",
        c"_append_ext_mod",
    )?;

    _append_ext_mod
        .getattr("append_ext_mod")?
        .call1((ext_mod,))?;

    Ok(())
}

pub fn write_py_err_to_file(
    py: Python<'_>,
    py_err: &PyErr,
    file_path: impl AsRef<Path>,
) -> io::Result<()> {
    if let Some(tb) = py_err.traceback(py) {
        if let Ok(tb) = tb.format() {
            return fs::write(file_path, tb);
        }
    }
    fs::write(file_path, format!("{:?}", py_err))
}
