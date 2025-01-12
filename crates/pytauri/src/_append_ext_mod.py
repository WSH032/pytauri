import sys
from importlib.metadata import EntryPoint, distribution, entry_points
from os import getenv
from multiprocessing import freeze_support
from types import ModuleType

__all__ = ["append_ext_mod"]

_SPECIFIC_DIST = getenv("_PYTAURI_DIST")
"""specify the package distribution name of a pytauri app to load the extension module."""


def _get_ext_mod_entry_point() -> EntryPoint:
    group = "pytauri"
    name = "ext_mod"

    if not _SPECIFIC_DIST:
        if sys.version_info >= (3, 10):
            # To avoid deprecation warnings
            eps = entry_points(group=group, name=name)
        else:
            # TODO: how to specify the name?
            eps = entry_points()[group]
    else:
        eps = distribution(_SPECIFIC_DIST).entry_points

    # we dont check if `len(eps) > 1`, `pytauri.ffi._ext_mod` will do that
    ep = next(iter(eps), None)
    if ep is None:
        raise RuntimeError(f"No `group={group}, name={name}` entry point is found")
    return ep


def append_ext_mod(ext_mod: ModuleType) -> None:
    ext_mod_path = _get_ext_mod_entry_point().value

    sys.modules[ext_mod_path] = ext_mod

    # See: <https://pyinstaller.org/en/stable/common-issues-and-pitfalls.html#multi-processing>
    #
    # > A typical symptom of failing to call multiprocessing.freeze_support()
    # > before your code (or 3rd party code you are using) attempts to make use of
    # > multiprocessing functionality is an endless spawn loop of your application process.
    #
    # So we do it for users automatically.
    #
    # NOTE: MUST use after `sys.modules[ext_mod_path] = ext_mod`,
    # or forked interpreter will not be able to import the module
    # (because the module in only in memory, not in the filesystem).
    #
    # NOTE: `freeze_support` only supports Windows with `spawn`.
    # But for unix, we have already set `fork` start method in `_freeze.py`,
    # so is's okay.
    freeze_support()
