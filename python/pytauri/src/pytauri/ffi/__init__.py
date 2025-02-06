"""Original FFI interface module.

!!! warning
    All APIs under this module should not be considered stable.
    You should use the re-exported APIs under the top-level module.
"""

from pytauri.ffi._ext_mod import EXT_MOD
from pytauri.ffi.lib import (
    App,
    AppHandle,
    Builder,
    BuilderArgs,
    Context,
    Emitter,
    Event,
    EventId,
    EventTarget,
    EventTargetType,
    ImplEmitter,
    ImplListener,
    ImplManager,
    Listener,
    Manager,
    RunEvent,
    RunEventType,
    builder_factory,
    context_factory,
)

__all__ = (
    "EXT_MOD",
    "App",
    "AppHandle",
    "Builder",
    "BuilderArgs",
    "Context",
    "Emitter",
    "Event",
    "EventId",
    "EventTarget",
    "EventTargetType",
    "ImplEmitter",
    "ImplListener",
    "ImplManager",
    "Listener",
    "Manager",
    "RunEvent",
    "RunEventType",
    "builder_factory",
    "context_factory",
)
