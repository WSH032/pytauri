"""[tauri::webview](https://docs.rs/tauri/latest/tauri/webview/index.html)"""

from typing import (
    TYPE_CHECKING,
    final,
)

from pytauri.ffi._ext_mod import pytauri_mod

__all__ = ["WebviewWindow"]

_webview_mod = pytauri_mod.webview

if TYPE_CHECKING:

    @final
    class WebviewWindow:
        """[tauri::webview::WebviewWindow](https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindow.html)"""

        def hide(self) -> None:
            """Hide this window."""

        def show(self) -> None:
            """Show this window."""

        def eval(self, js: str, /) -> None:
            """Evaluates JavaScript on this window."""
else:
    WebviewWindow = _webview_mod.WebviewWindow
