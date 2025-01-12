import sys
from importlib.metadata import entry_points
from multiprocessing import freeze_support
from types import ModuleType

__all__ = ["append_ext_mod"]


def append_ext_mod(ext_mod: ModuleType) -> None:
    if sys.version_info >= (3, 10):
        # To avoid deprecation warnings
        eps = entry_points(group="pytauri", name="ext_mod")
    else:
        # TODO: how to specify the name?
        eps = entry_points()["pytauri"]

    ext_mod_path = next(iter(eps)).value

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
