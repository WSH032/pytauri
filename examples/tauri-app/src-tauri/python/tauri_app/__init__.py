# ruff: noqa: D101, D103

"""The tauri-app."""

from os import environ

# This is an env var that can only be used internally by pytauri to distinguish
# between different example extension modules.
# You don't need and shouldn't set this in your own app.
# Must be set before importing any pytauri module.
environ["_PYTAURI_DIST"] = "tauri-app"

import sys

from anyio.from_thread import start_blocking_portal
from pydantic import BaseModel
from pytauri import (
    AppHandle,
    BuilderArgs,
    Commands,
    builder_factory,
    context_factory,
)
from pytauri_plugin_notification import NotificationBuilderArgs, NotificationExt

commands: Commands = Commands()


class Person(BaseModel):
    name: str


class Greeting(BaseModel):
    message: str


@commands.command()
async def greet(body: Person, app_handle: AppHandle) -> Greeting:
    notification_builder = NotificationExt.builder(app_handle)
    notification_builder.show(
        NotificationBuilderArgs(title="Greeting", body=f"Hello, {body.name}!")
    )

    return Greeting(
        message=f"Hello, {body.name}! You've been greeted from Python {sys.version}!"
    )


def main() -> None:
    """Run the tauri-app."""
    with start_blocking_portal("asyncio") as portal:  # or `trio`
        app = builder_factory().build(
            BuilderArgs(
                context=context_factory(),
                invoke_handler=commands.generate_handler(portal),
            )
        )
        app.run()
