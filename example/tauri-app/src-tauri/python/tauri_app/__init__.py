# ruff: noqa: D101, D103

"""The tauri-app."""

from anyio.from_thread import start_blocking_portal
from pydantic import BaseModel
from pytauri import (
    BuilderArgs,
    Commands,
    builder_factory,
    context_factory,
)

commands: Commands = Commands()


class Person(BaseModel):
    name: str


class Greeting(BaseModel):
    message: str


@commands.command()
async def greet(body: Person) -> Greeting:
    return Greeting(message=f"Hello, {body.name}! You've been greeted from Python!")


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
