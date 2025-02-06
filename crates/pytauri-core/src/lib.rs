// See: <https://doc.rust-lang.org/rustdoc/unstable-features.html#extensions-to-the-doc-attribute>
#![cfg_attr(
    docsrs,
    feature(doc_cfg, doc_auto_cfg, doc_cfg_hide),
    doc(cfg_hide(doc))
)]

mod ext_mod_impl;
pub mod tauri_runtime;
pub mod utils;

use pyo3::prelude::*;

/// You can access this module in Python via `pytuari.EXT_MOD.pytuari`.
///
/// See also: [tauri]
#[pymodule(submodule, gil_used = false, name = "pytauri")]
pub mod ext_mod {
    use super::*;

    #[pymodule_export]
    pub use ext_mod_impl::{App, AppHandle, Context, Event, Listener, Manager, RunEvent};

    pub use ext_mod_impl::{
        EventId, ImplListener, ImplManager, PyAppHandleExt, PyAppHandleStateError,
        PyAppHandleStateResult,
    };

    /// see also: [tauri::ipc]
    #[pymodule]
    pub mod ipc {
        use super::*;

        #[pymodule_export]
        pub use ext_mod_impl::ipc::{Channel, Invoke, InvokeResolver, JavaScriptChannelId};
    }

    /// see also: [tauri::webview]
    #[pymodule]
    pub mod webview {
        use super::*;

        #[pymodule_export]
        pub use ext_mod_impl::webview::{Webview, WebviewWindow};
    }

    /// see also: [tauri::path]
    #[pymodule]
    pub mod path {
        use super::*;

        #[pymodule_export]
        pub use ext_mod_impl::path::PathResolver;
    }
}
