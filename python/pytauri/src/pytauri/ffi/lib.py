"""[tauri::self](https://docs.rs/tauri/latest/tauri/index.html)"""

from typing import (
    TYPE_CHECKING,
    Any,
    Callable,
    NewType,
    Optional,
    Protocol,
    Union,
    final,
)

from typing_extensions import Self, TypeAlias

from pytauri.ffi._ext_mod import pytauri_mod

__all__ = [
    "App",
    "AppHandle",
    "Builder",
    "BuilderArgs",
    "Context",
    "Event",
    "EventId",
    "ImplListener",
    "ImplManager",
    "Listener",
    "Manager",
    "RunEvent",
    "RunEventType",
    "builder_factory",
    "context_factory",
]

if TYPE_CHECKING:
    from pytauri.ffi.ipc import Invoke


class _InvokeHandlerProto(Protocol):
    def __call__(self, invoke: "Invoke", /) -> Any: ...


_AppRunCallbackType = Callable[["AppHandle", "RunEventType"], None]

_EventHandlerType = Callable[["Event"], None]


if TYPE_CHECKING:
    from pytauri.ffi import path, webview

    @final
    class App:
        """[Tauri::app](https://docs.rs/tauri/latest/tauri/struct.App.html)

        !!! warning
            This class is not thread-safe, and should not be shared between threads.

            - You can only use it on the thread it was created on.
            - And you need to ensure it is garbage collected on the thread it was created on,
                otherwise it will cause memory leaks.
        """

        def run(self, callback: Optional[_AppRunCallbackType] = None, /) -> None:
            """Consume and run this app, will block until the app is exited.

            Args:
                callback: a callback function that will be called on each event.
                    It will be called on the same thread that the app was created on,
                    so you should not block in this function.

            !!! warning
                If `callback` is specified, it must not raise an exception,
                otherwise it is undefined behavior, and in most cases, the program will panic.
            """

        def run_iteration(
            self, callback: Optional[_AppRunCallbackType] = None, /
        ) -> None:
            """Run this app iteratively without consuming it, calling `callback` on each iteration.

            Args:
                callback: a callback function that will be called on each iteration.

            !!! warning
                `callback` has the same restrictions as [App.run][pytauri.App.run].

            !!! tip
                Approximately 2ms per calling in debug mode.
            """

        def cleanup_before_exit(self, /) -> None:
            """Runs necessary cleanup tasks before exiting the process.

            **You should always exit the tauri app immediately after this function returns and not use any tauri-related APIs.**
            """

        def handle(self, /) -> "AppHandle":
            """Get a handle to this app, which can be used to interact with the app from another thread."""
            ...

    @final
    class AppHandle:
        """[tauri::AppHandle](https://docs.rs/tauri/latest/tauri/app/struct.AppHandle.html)"""

    @final
    class BuilderArgs:  # noqa: D101
        def __new__(
            cls,
            /,
            *,
            context: "Context",
            invoke_handler: Optional[_InvokeHandlerProto] = None,
        ) -> Self:
            """[tauri::Builder](https://docs.rs/tauri/latest/tauri/struct.Builder.html)

            !!! warning
                The implementer of `invoke_handler` must never raise an exception,
                otherwise it is considered undefined behavior.
                Additionally, `invoke_handler` must not block.

            Args:
                context: use [context_factory][pytauri.context_factory] to get it.
                invoke_handler: use [Commands][pytauri.ipc.Commands] to get it.
            """
            ...

    @final
    class Builder:
        """[Tauri::Builder](https://docs.rs/tauri/latest/tauri/struct.Builder.html)

        use [builder_factory][pytauri.builder_factory] to instantiate this class.

        !!! warning
            This class is not thread-safe, and should not be shared between threads.

            - You can only use it on the thread it was created on.
            - And you need to ensure it is garbage collected on the thread it was created on,
                otherwise it will cause memory leaks.
        """

        def build(self, args: BuilderArgs, /) -> App:
            """Consume this builder and build an app with the given `BuilderArgs`."""
            ...

    @final
    class Context:
        """[tauri::Context](https://docs.rs/tauri/latest/tauri/struct.Context.html)"""

    @final
    class RunEvent:
        """[tauri::RunEvent](https://docs.rs/tauri/latest/tauri/enum.RunEvent.html)"""

        @final
        class Exit:
            """[tauri::RunEvent::Exit](https://docs.rs/tauri/latest/tauri/enum.RunEvent.html#variant.Exit)"""

        @final
        class ExitRequested:
            """[tauri::RunEvent::ExitRequested](https://docs.rs/tauri/latest/tauri/enum.RunEvent.html#variant.ExitRequested)"""

            code: Optional[int]

        @final
        class WindowEvent:
            """[tauri::RunEvent::WindowEvent](https://docs.rs/tauri/latest/tauri/enum.RunEvent.html#variant.WindowEvent)"""

            label: str

        @final
        class WebviewEvent:
            """[tauri::RunEvent::WebviewEvent](https://docs.rs/tauri/latest/tauri/enum.RunEvent.html#variant.WebviewEvent)"""

            label: str

        @final
        class Ready:
            """[tauri::RunEvent::Ready](https://docs.rs/tauri/latest/tauri/enum.RunEvent.html#variant.Ready)"""

        @final
        class Resumed:
            """[tauri::RunEvent::Resumed](https://docs.rs/tauri/latest/tauri/enum.RunEvent.html#variant.Resumed)"""

        @final
        class MainEventsCleared:
            """[tauri::RunEvent::MainEventsCleared](https://docs.rs/tauri/latest/tauri/enum.RunEvent.html#variant.MainEventsCleared)"""

        @final
        class MenuEvent:
            """[tauri::RunEvent::MenuEvent](https://docs.rs/tauri/latest/tauri/enum.RunEvent.html#variant.MenuEvent)"""

    def builder_factory(*args: Any, **kwargs: Any) -> Builder:
        """A factory function for creating a `Builder` instance.

        This is the closure passed from the Rust side when initializing the pytauri pyo3 module.
        `args` and `kwargs` will be passed to this closure.
        """
        ...

    def context_factory(*args: Any, **kwargs: Any) -> Context:
        """A factory function for creating a `Context` instance.

        This is the closure passed from the Rust side when initializing the pytauri pyo3 module.
        `args` and `kwargs` will be passed to this closure.
        """
        ...

    @final
    class Manager:
        """[tauri::Manager](https://docs.rs/tauri/latest/tauri/trait.Manager.html)"""

        @staticmethod
        def app_handle(slf: "ImplManager", /) -> AppHandle:
            """The application handle associated with this manager."""
            ...

        @staticmethod
        def get_webview_window(
            slf: "ImplManager", label: str, /
        ) -> Optional[webview.WebviewWindow]:
            """Fetch a single webview window from the manager."""
            ...

        @staticmethod
        def webview_windows(slf: "ImplManager", /) -> dict[str, webview.WebviewWindow]:
            """Fetch all managed webview windows."""
            ...

        @staticmethod
        def path(slf: "ImplManager", /) -> path.PathResolver:
            """The path resolver is a helper class for general and application-specific path APIs."""
            ...

    class Event:
        """[tauri::Event](https://docs.rs/tauri/latest/tauri/struct.Event.html)"""

        @property
        def id(self) -> "EventId":
            """The `EventId` of the handler that was triggered."""
            ...

        @property
        def payload(self) -> str:
            """The event payload."""
            ...

    class Listener:
        """[tauri::Listener](https://docs.rs/tauri/latest/tauri/trait.Listener.html)

        See also: <https://tauri.app/develop/calling-rust/#event-system>
        """

        @staticmethod
        def listen(
            slf: "ImplListener",
            event: str,
            handler: _EventHandlerType,
            /,
        ) -> "EventId":
            """Listen to an emitted event on this manager.

            !!! warning
                `handler` has the same restrictions as [App.run][pytauri.App.run].
            """
            ...

        @staticmethod
        def once(
            slf: "ImplListener",
            event: str,
            handler: _EventHandlerType,
            /,
        ) -> "EventId":
            """Listen to an event on this manager only once.

            !!! warning
                `handler` has the same restrictions as [App.run][pytauri.App.run].
            """
            ...

        @staticmethod
        def unlisten(
            slf: "ImplListener",
            id: "EventId",  # noqa: A002
            /,
        ) -> None:
            """Remove an event listener."""
            ...

        @staticmethod
        def listen_any(
            slf: "ImplListener",
            event: str,
            handler: _EventHandlerType,
            /,
        ) -> "EventId":
            """Listen to an emitted event to any target.

            !!! warning
                `handler` has the same restrictions as [App.run][pytauri.App.run].
            """
            ...

        @staticmethod
        def once_any(
            slf: "ImplListener",
            event: str,
            handler: _EventHandlerType,
            /,
        ) -> "EventId":
            """Listens once to an emitted event to any target .

            !!! warning
                `handler` has the same restrictions as [App.run][pytauri.App.run].
            """
            ...


else:
    App = pytauri_mod.App
    AppHandle = pytauri_mod.AppHandle
    Builder = pytauri_mod.Builder
    BuilderArgs = pytauri_mod.BuilderArgs
    Context = pytauri_mod.Context
    RunEvent = pytauri_mod.RunEvent
    builder_factory = pytauri_mod.builder_factory
    context_factory = pytauri_mod.context_factory
    Manager = pytauri_mod.Manager
    Event = pytauri_mod.Event
    Listener = pytauri_mod.Listener


RunEventType: TypeAlias = Union[
    RunEvent.Exit,
    RunEvent.ExitRequested,
    RunEvent.WindowEvent,
    RunEvent.WebviewEvent,
    RunEvent.Ready,
    RunEvent.Resumed,
    RunEvent.MainEventsCleared,
    RunEvent.MenuEvent,
]
"""See [RunEvent][pytauri.ffi.RunEvent] for details."""

ImplManager: TypeAlias = Union[App, AppHandle, "webview.WebviewWindow"]

EventId = NewType("EventId", int)
"""[tauri::EventId](https://docs.rs/tauri/latest/tauri/type.EventId.html)"""

ImplListener = ImplManager
