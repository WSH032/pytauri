# ruff: noqa: D101, D103, D104, D107

from os import environ

# This is an env var that can only be used internally by pytauri to distinguish
# between different example extension modules.
# You don't need and shouldn't set this in your own app.
# Must be set before importing any pytauri module.
environ["_PYTAURI_DIST"] = "nicegui-app"

import sys
from concurrent.futures import Future
from socket import socket
from threading import Event
from typing import Any, Optional

import uvicorn
from anyio.from_thread import start_blocking_portal
from fastapi import FastAPI
from nicegui import ui
from pytauri import (
    AppHandle,
    BuilderArgs,
    Manager,
    RunEvent,
    RunEventEnum,
    builder_factory,
    context_factory,
)
from pytauri_plugin_notification import NotificationBuilderArgs, NotificationExt
from typing_extensions import override


class FrontServer(uvicorn.Server):
    """Override `uvicorn.Server` to set events on startup and shutdown."""

    def __init__(
        self, config: uvicorn.Config, *, startup_event: Event, shutdown_event: Event
    ) -> None:
        super().__init__(config)
        self.startup_event = startup_event
        self.shutdown_event = shutdown_event

    @override
    async def startup(self, sockets: Optional[list[socket]] = None) -> None:
        await super().startup(sockets)
        self.startup_event.set()

    @override
    async def serve(self, sockets: Optional[list[socket]] = None) -> None:
        await super().serve(sockets)
        self.shutdown_event.set()

    def request_shutdown(self) -> None:
        """Request the server to shutdown.

        Note:
            This method is not thread-safe.

        Ref:
            - <https://github.com/zauberzeug/nicegui/discussions/1957#discussioncomment-7484548>
            - <https://github.com/encode/uvicorn/discussions/1103#discussioncomment-6187606>
        """
        self.should_exit = True


def init_ui(app_handle: AppHandle) -> None:
    async def greet():
        notification_builder = NotificationExt.builder(app_handle)
        notification_builder.show(
            NotificationBuilderArgs(title="Greeting", body=f"Hello, {name.value}!")
        )

        message.set_text(
            f"Hello, {name.value}! You've been greeted from Python {sys.version}!"
        )

    with ui.row():
        name = ui.input("Enter a name...")
        ui.button("Greet").on_click(greet)
    message = ui.label()


def main() -> None:
    nicegui_app = FastAPI()
    ui.run_with(nicegui_app)
    server_startup_event = Event()
    server_shutdown_event = Event()
    server = FrontServer(
        # `host` and `port` are the same as `frontendDist` in `tauri.conf.json`
        uvicorn.Config(nicegui_app, host="localhost", port=8080),
        shutdown_event=server_shutdown_event,
        startup_event=server_startup_event,
    )

    with start_blocking_portal("asyncio") as portal:  # or `trio`
        server_exception: Optional[BaseException] = None

        def server_failed_callback(future: Future[Any]) -> None:
            nonlocal server_exception
            server_exception = future.exception()
            if server_exception is not None:
                # server startup failed, so we must set these events manually for tauri app
                server_startup_event.set()
                server_shutdown_event.set()

        # launch the front server
        portal.start_task_soon(server.serve).add_done_callback(server_failed_callback)

        tauri_app = builder_factory().build(
            BuilderArgs(
                context=context_factory(),
            )
        )

        init_ui(tauri_app.handle())

        def tauri_run_callback(app_handle: AppHandle, run_event: RunEvent) -> None:
            run_event_enum = run_event.match_ref()

            # show the main window after the server is started
            if isinstance(run_event_enum, RunEventEnum.Ready):
                window = Manager.get_webview_window(app_handle, "main")
                assert (
                    window is not None
                ), "you forgot to set the unvisible 'main' window in `tauri.conf.json`"

                server_startup_event.wait()
                window.show()
                if (
                    server_exception is not None
                    or server.should_exit  # `nicegui_app` asgi lifespan startup failed
                ):
                    window.eval(
                        "document.body.innerHTML = `failed to start front server, see backend logs for details`"
                    )
            # wait for the server to shutdown
            elif isinstance(run_event_enum, RunEventEnum.Exit):
                server.request_shutdown()
                server_shutdown_event.wait()

        tauri_app.run(tauri_run_callback)
