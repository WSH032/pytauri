# Build python source (sdist) distribution

## setuptools-rust

When you want to distribute your app in Python format, you need to compile pytauri into a Python extension module file, instead of providing it in memory through `pytauri::standalone::append_ext_mod` in the `main.rs` executable.

To do this, we need to use [setuptools-rust](https://github.com/PyO3/setuptools-rust).

Add it to `[build-system]`:

```toml title="src-tauri/pyproject.toml"
[build-system]
requires = ["setuptools>=61", "setuptools-rust==1.*"]
build-backend = "setuptools.build_meta"
```

And add the following file:

```python title="src-tauri/setup.py"
"""See: <https://setuptools-rust.readthedocs.io/en/latest/setuppy_tutorial.html>"""

from os import getenv

from setuptools import setup
from setuptools_rust import RustExtension

PYTAURI_STANDALONE = getenv("PYTAURI_STANDALONE") == "1"
"""Instead of building pytauri as an extension module file, it will be loaded in memory through Rust's `append_ext_mod`"""

setup(
    rust_extensions=[
        RustExtension(
            # set `target` the same as `[project.entry-points.pytauri.ext_mod]` in `pyproject.toml`
            target="tauri_app.ext_mod",
            # It is recommended to set other features in `Cargo.toml`, except the following features:
            features=[
                # see: <https://pyo3.rs/v0.23.3/building-and-distribution.html#the-extension-module-feature>,
                # required to build the extension module
                "pyo3/extension-module",
                # This feature tells Tauri to use embedded frontend assets instead of using a frontend development server.
                # Usually, this feature is enabled by `tauri-cli`, here we enable it manually.
                "tauri/custom-protocol",
            ],
        )
    ]
    if not PYTAURI_STANDALONE
    else [],
)
```

## Include frontend assets

You need to include the frontend assets in the sdist so that users can build your app from the source.

Configure Vite to output the frontend assets to `src-tauri/frontend`:

```ts title="vite.config.ts"
// https://vitejs.dev/config/
export default defineConfig(async () => ({
  build: {
    // build dist into `src-tauri/` so that we can include it in Python sdist
    outDir: "src-tauri/frontend",
  },
}));
```

Also, let tauri-cli know where the frontend assets are:

```json title="src-tauri/tauri.conf.json"
{
  "build": {
    "frontendDist": "./frontend"
  },
}
```

Include the frontend assets in the sdist:

ref: <https://setuptools.pypa.io/en/latest/userguide/miscellaneous.html>

```title="src-tauri/MANIFEST.in"
graft frontend/
```

## Include rust files

You will also need to tell Setuptools that the Rust files are required to build your project from the source distribution. That can be done either via `MANIFEST.in` or via a plugin like [setuptools-scm](https://github.com/pypa/setuptools-scm).

=== "setuptools-scm"

    Use `setuptools-scm` to include all files tracked by `git` (just add it as a dependency):

    ```toml title="src-tauri/pyproject.toml"
    [build-system]
    requires = ["setuptools>=61", "setuptools-rust==1.*", "setuptools_scm>=8"]
    build-backend = "setuptools.build_meta"
    ```

    !!! info
        Normally, we do not track `src-tauri/frontend`, which is why we use `MANIFEST.in` to include it.

=== "MANIFEST.in"

    ```title="src-tauri/MANIFEST.in"
    graft frontend/
    graft capabilities/
    graft icons/
    graft src/
    include Cargo.toml
    include Cargo.lock
    include build.rs
    include tauri.conf.json
    ```

## Build sdist

- Build frontend assets first: `#!bash pnpm build` (see `build.beforeBuildCommand` in `tau.conf.json`).

- Now you can build only sdist: `#!bash uv build src-tauri --sdist`.

    - Or build both wheel and sdist: `#!bash uv build src-tauri`.

    !!! tip
        As long as you can build the wheel with `#!bash uv build src-tauri`, it means your Python sdist can be used normally. Otherwise, you might have forgotten to include some Rust files.