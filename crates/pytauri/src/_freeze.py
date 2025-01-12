import sys
from multiprocessing import set_executable, set_start_method

__all__ = ["freeze"]


def freeze(current_exe: str, /) -> None:
    # ref:
    #
    # - <https://gregoryszorc.com/docs/pyoxidizer/main/pyoxidizer_packaging_multiprocessing.html>
    # - <https://github.com/python/cpython/blob/60403a5409ff2c3f3b07dd2ca91a7a3e096839c7/Lib/multiprocessing/spawn.py#L67-L95>
    # - <https://pyinstaller.org/en/stable/common-issues-and-pitfalls.html#multi-processing>
    # - <https://github.com/pyinstaller/pyinstaller/blob/v6.11.1/PyInstaller/hooks/rthooks/pyi_rth_multiprocessing.py>

    # NOTE: Even if we do not intend to support the `multiprocessing`,
    # it is necessary to set `sys.frozen` to `True` so that the Python side can
    # recognize that `pytauri` is in `standalone` mode
    setattr(sys, "frozen", True)  # noqa: B010

    if sys.platform == "win32":
        set_start_method("spawn")
    else:
        # Because `freeze_support` only supports Windows with `spawn`,
        # so we must use `fork` on unix, or we will get an
        # endless spawn loop of the application process.
        # See: <https://pyinstaller.org/en/stable/common-issues-and-pitfalls.html#multi-processing>
        #
        # We must set it munaually here, because the default value is:
        # - MacOs: `spawn`
        # - Linux: `forkserver` if `sys.version_info >= (3, 14)` else `fork`
        set_start_method("fork")

    set_executable(current_exe)
