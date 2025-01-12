"""Original FFI interface module.

!!! warning
    All APIs under this module should not be considered stable.
    You should use the re-exported APIs under the top-level module.
"""

from types import ModuleType
from typing import TYPE_CHECKING, Optional, final

from pytauri import EXT_MOD, ImplManager
from typing_extensions import Self, TypeAlias

__all__ = [
    "ImplNotificationExt",
    "NotificationBuilder",
    "NotificationBuilderArgs",
    "NotificationExt",
]


def _load_notification_mod(ext_mod: ModuleType) -> ModuleType:
    try:
        notification_mod = ext_mod.notification
    except AttributeError as e:
        raise RuntimeError(
            "Submodule `notification` is not found in the extension module"
        ) from e

    assert isinstance(notification_mod, ModuleType)
    return notification_mod


_notification_mod = _load_notification_mod(EXT_MOD)

if TYPE_CHECKING:

    @final
    class NotificationBuilderArgs:  # noqa: D101
        def __new__(
            cls, /, *, title: Optional[str] = None, body: Optional[str] = None
        ) -> Self:
            """[tauri_plugin_notification::NotificationBuilder](https://docs.rs/tauri-plugin-notification/latest/tauri_plugin_notification/struct.NotificationBuilder.html)"""
            ...

    @final
    class NotificationBuilder:
        """[tauri_plugin_notification::NotificationBuilder](https://docs.rs/tauri-plugin-notification/latest/tauri_plugin_notification/struct.NotificationBuilder.html)"""

        def show(self, args: NotificationBuilderArgs, /) -> None:
            """Consume this builder and show the notification.

            # FIXME, XXX, TODO:

            See: <https://github.com/tauri-apps/tauri/issues/3700>

            On windows, you must install the package via the `.msi` or `nsis`, or `tauri-plugin-notification` will not work.

            Tracker issue: <https://github.com/tauri-apps/plugins-workspace/issues/2156>
            """

    @final
    class NotificationExt:
        """[tauri_plugin_notification::NotificationExt](https://docs.rs/tauri-plugin-notification/latest/tauri_plugin_notification/trait.NotificationExt.html)"""

        @staticmethod
        def builder(slf: "ImplNotificationExt", /) -> NotificationBuilder:
            """Create a new notification builder."""
            ...

else:
    NotificationBuilderArgs = _notification_mod.NotificationBuilderArgs
    NotificationBuilder = _notification_mod.NotificationBuilder
    NotificationExt = _notification_mod.NotificationExt

ImplNotificationExt: TypeAlias = ImplManager
"""The implementors of `NotificationExt`."""
