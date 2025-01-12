// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// From: <https://crates.io/crates/pyembed/0.24.0>

use std::path::Path;

use libc::wchar_t;
use pyo3::ffi as pyffi;

// see: <https://github.com/indygreg/PyOxidizer/blob/1ceca8664c71f39e849ce4873e00d821504b32bd/pyembed/src/error.rs>
mod error {
    use {
        pyo3::{ffi as pyffi, prelude::*},
        std::{
            ffi::CStr,
            fmt::{Display, Formatter},
        },
    };

    /// Format a PyErr in a crude manner.
    ///
    /// This is meant to be called during interpreter initialization. We can't
    /// call PyErr_Print() because sys.stdout may not be available yet.
    fn format_pyerr(py: Python, err: PyErr) -> Result<String, &'static str> {
        let type_repr = err
            .get_type(py)
            .repr()
            .map_err(|_| "unable to get repr of error type")?;

        let value_repr = err
            .value(py)
            .repr()
            .map_err(|_| "unable to get repr of error value")?;

        let value = format!(
            "{}: {}",
            type_repr.to_string_lossy(),
            value_repr.to_string_lossy()
        );

        Ok(value)
    }

    /// Represents an error encountered when creating an embedded Python interpreter.
    #[derive(Debug)]
    pub enum NewInterpreterError {
        Simple(&'static str),
        Dynamic(String),
    }

    impl From<&'static str> for NewInterpreterError {
        fn from(v: &'static str) -> Self {
            NewInterpreterError::Simple(v)
        }
    }

    impl Display for NewInterpreterError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self {
                NewInterpreterError::Simple(value) => value.fmt(f),
                NewInterpreterError::Dynamic(value) => value.fmt(f),
            }
        }
    }

    impl std::error::Error for NewInterpreterError {}

    impl NewInterpreterError {
        pub fn new_from_pyerr(py: Python, err: PyErr, context: &str) -> Self {
            match format_pyerr(py, err) {
                Ok(value) => NewInterpreterError::Dynamic(format!("during {}: {}", context, value)),
                Err(msg) => NewInterpreterError::Dynamic(format!("during {}: {}", context, msg)),
            }
        }

        pub fn new_from_pystatus(status: &pyffi::PyStatus, context: &str) -> Self {
            if !status.func.is_null() && !status.err_msg.is_null() {
                let func = unsafe { CStr::from_ptr(status.func) };
                let msg = unsafe { CStr::from_ptr(status.err_msg) };

                NewInterpreterError::Dynamic(format!(
                    "during {}: {}: {}",
                    context,
                    func.to_string_lossy(),
                    msg.to_string_lossy()
                ))
            } else if !status.err_msg.is_null() {
                let msg = unsafe { CStr::from_ptr(status.err_msg) };

                NewInterpreterError::Dynamic(format!(
                    "during {}: {}",
                    context,
                    msg.to_string_lossy()
                ))
            } else {
                NewInterpreterError::Dynamic(format!(
                    "during {}: could not format PyStatus",
                    context
                ))
            }
        }
    }

    pub type NewInterpreterResult<T> = Result<T, NewInterpreterError>;
}

pub(crate) use error::{NewInterpreterError, NewInterpreterResult};

// see: <https://github.com/indygreg/PyOxidizer/blob/1ceca8664c71f39e849ce4873e00d821504b32bd/pyembed/src/interpreter_config.rs#L29-L251>
pub(crate) mod utils {
    use super::*;
    use std::ffi::OsString;

    #[cfg(target_family = "unix")]
    use std::{
        ffi::{CString, NulError},
        os::unix::ffi::OsStrExt,
    };

    #[cfg(target_family = "windows")]
    use std::os::windows::prelude::OsStrExt;

    #[cfg(unix)]
    pub(crate) fn set_config_string_from_path(
        config: &pyffi::PyConfig,
        dest: &*mut wchar_t,
        path: &Path,
        context: &str,
    ) -> Result<(), NewInterpreterError> {
        let value = CString::new(path.as_os_str().as_bytes())
            .map_err(|_| NewInterpreterError::Simple("cannot convert path to C string"))?;

        let status = unsafe {
            pyffi::PyConfig_SetBytesString(
                config as *const _ as *mut _,
                dest as *const *mut _ as *mut *mut _,
                value.as_ptr() as *const _,
            )
        };

        if unsafe { pyffi::PyStatus_Exception(status) } != 0 {
            Err(NewInterpreterError::new_from_pystatus(&status, context))
        } else {
            Ok(())
        }
    }

    #[cfg(windows)]
    pub(crate) fn set_config_string_from_path(
        config: &pyffi::PyConfig,
        dest: &*mut wchar_t,
        path: &Path,
        context: &str,
    ) -> Result<(), NewInterpreterError> {
        let status = unsafe {
            let mut value: Vec<wchar_t> = path.as_os_str().encode_wide().collect();
            // NULL terminate.
            value.push(0);

            pyffi::PyConfig_SetString(
                config as *const _ as *mut _,
                dest as *const *mut _ as *mut *mut _,
                value.as_ptr() as *const _,
            )
        };

        if unsafe { pyffi::PyStatus_Exception(status) } != 0 {
            Err(NewInterpreterError::new_from_pystatus(&status, context))
        } else {
            Ok(())
        }
    }

    #[cfg(target_family = "unix")]
    pub fn set_argv(
        config: &mut pyffi::PyConfig,
        args: &[OsString],
    ) -> Result<(), NewInterpreterError> {
        let argc = args.len() as isize;
        let argv = args
            .iter()
            .map(|x| CString::new(x.as_bytes()))
            .collect::<Result<Vec<_>, NulError>>()
            .map_err(|_| {
                NewInterpreterError::Simple("unable to construct C string from OsString")
            })?;
        let argvp = argv
            .iter()
            .map(|x| x.as_ptr() as *mut i8)
            .collect::<Vec<_>>();

        let status = unsafe {
            pyffi::PyConfig_SetBytesArgv(config as *mut _, argc, argvp.as_ptr() as *mut _)
        };

        if unsafe { pyffi::PyStatus_Exception(status) } != 0 {
            Err(NewInterpreterError::new_from_pystatus(
                &status,
                "setting argv",
            ))
        } else {
            Ok(())
        }
    }

    #[cfg(target_family = "windows")]
    pub fn set_argv(
        config: &mut pyffi::PyConfig,
        args: &[OsString],
    ) -> Result<(), NewInterpreterError> {
        let argc = args.len() as isize;
        let argv = args
            .iter()
            .map(|x| {
                let mut buffer = x.encode_wide().collect::<Vec<u16>>();
                buffer.push(0);

                buffer
            })
            .collect::<Vec<_>>();
        let argvp = argv
            .iter()
            .map(|x| x.as_ptr() as *mut u16)
            .collect::<Vec<_>>();

        let status =
            unsafe { pyffi::PyConfig_SetArgv(config as *mut _, argc, argvp.as_ptr() as *mut _) };

        if unsafe { pyffi::PyStatus_Exception(status) } != 0 {
            Err(NewInterpreterError::new_from_pystatus(
                &status,
                "setting argv",
            ))
        } else {
            Ok(())
        }
    }
}
