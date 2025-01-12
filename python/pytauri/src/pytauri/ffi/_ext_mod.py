from os import getenv
from types import ModuleType
from typing import TYPE_CHECKING

# See: <https://pypi.org/project/backports.entry-points-selectable/>
# and: <https://docs.python.org/3/library/importlib.metadata.html#entry-points>
# Deprecated: once we no longer support versions Python 3.9, we can remove this dependency.
from importlib_metadata import (
    EntryPoint,
    distribution,
    entry_points,  # pyright: ignore[reportUnknownVariableType]
)

__all__ = ["EXT_MOD", "pytauri_mod"]

_SPECIFIC_DIST = getenv("_PYTAURI_DIST")
"""specify the package distribution name of a pytauri app to load the extension module."""


def _load_ext_mod() -> ModuleType:
    group = "pytauri"
    name = "ext_mod"
    eps = (
        entry_points(group=group, name=name)
        if not _SPECIFIC_DIST
        else distribution(_SPECIFIC_DIST).entry_points.select(group=group, name=name)  # pyright: ignore[reportUnknownMemberType]
    )
    eps: tuple[EntryPoint, ...] = tuple(eps)

    if len(eps) == 0:
        raise RuntimeError("No `pytauri` entry point is found")
    elif len(eps) > 1:
        msg_list: list[tuple[str, str]] = []
        for ep in eps:
            # See: <https://packaging.python.org/en/latest/specifications/core-metadata/#core-metadata>
            # for more attributes of `dist`.
            name = ep.dist.name if ep.dist else "UNKNOWN"
            msg_list.append((name, repr(ep)))

        prefix = "\n    - "
        msg = prefix.join(f"{name}: {ep}" for name, ep in msg_list)
        raise RuntimeError(
            f"Exactly one `pytauri` entry point is expected, but got:{prefix}{msg}"
        )

    ext_mod = eps[0].load()
    assert isinstance(ext_mod, ModuleType)

    return ext_mod


def _load_pytauri_mod(ext_mod: ModuleType) -> ModuleType:
    try:
        pytauri_mod = ext_mod.pytauri
    except AttributeError as e:
        raise RuntimeError(
            "submodule `pytauri` is not found in the extension module"
        ) from e

    assert isinstance(pytauri_mod, ModuleType)
    return pytauri_mod


if TYPE_CHECKING:
    EXT_MOD: ModuleType
    """The extension module of `pytauri` app.

    It will be loaded from `entry_points(group="pytauri", name="ext_mod")`.

    Usually you don't need to use it, unless you want to write plugins for `pytauri`.
    """
    pytauri_mod: ModuleType
    """The python module of `pytauri`.

    Equivalent to `EXT_MOD.pytauri`.
    """

else:
    EXT_MOD = _load_ext_mod()
    pytauri_mod = _load_pytauri_mod(EXT_MOD)
