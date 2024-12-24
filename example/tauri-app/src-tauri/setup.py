"""See: <https://setuptools-rust.readthedocs.io/en/latest/setuppy_tutorial.html>"""

from os import getenv

from setuptools import setup
from setuptools_rust import RustExtension

PYTAURI_STANDALONE = getenv("PYTAURI_STANDALONE") == "1"
"""Instead of building pytauri as a extension module file, it will be loaded in memory through Rust's `append_ext_mod`"""

setup(
    rust_extensions=[
        RustExtension(
            # set `target` the same as `[project.entry-points.pytauri.ext_mod]` in `pyproject.toml`
            target="tauri_app.ext_mod",
            # It is recommended to set other features in `Cargo.toml`, except following features:
            features=[
                # see: <https://pyo3.rs/v0.23.3/building-and-distribution.html#the-extension-module-feature>,
                # required to build the extension module
                "pyo3/extension-module",
                # This feature tells Tauri to use embedded frontend assets instead of using a frontend development server.
                # Usually this feature is enabled by `tauri-cli`, here we enable it manually.
                "tauri/custom-protocol",
            ],
        )
    ]
    if not PYTAURI_STANDALONE
    else [],
)
