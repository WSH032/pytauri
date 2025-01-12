<!-- The content will be also use in `docs/index.md` by `pymdownx.snippets` -->
<!-- Do not use any **relative link** and  **GitHub-specific syntax** ！-->
<!-- Do not rename or move the file -->

# PyTauri

[Tauri] bindings for Python through [Pyo3]

[Tauri]: https://github.com/tauri-apps/tauri
[Pyo3]: https://github.com/PyO3/pyo3

---

[![CI: docs]][CI: docs#link]

Documentation: <https://wsh032.github.io/pytauri/>

Source Code: <https://github.com/WSH032/pytauri/>

[CI: docs]: https://github.com/WSH032/pytauri/actions/workflows/docs.yml/badge.svg
[CI: docs#link]: https://github.com/WSH032/pytauri/actions/workflows/docs.yml

---

## Features

> **TL;DR**
>
> You are hurry and just wanna see/run the demo? See [examples/tauri-app](https://github.com/WSH032/pytauri/tree/main/examples/tauri-app).

[notification]: https://docs.rs/tauri-plugin-notification/latest/tauri_plugin_notification/

- Need Rust compiler, but **almost don't need to write Rust code**!
- Can be integrated with `tauri-cli` to build and package standalone executables!
- No IPC (inter-process communication) overhead, secure and fast, thanks to [Pyo3]!
- Support Tauri official plugins(e.g., [notification]), and you can write your own plugins!

    ![demo](https://github.com/user-attachments/assets/14ad5b51-b333-4d80-b04b-af72c4179571)

- Natively support async python (`asyncio`, `trio` or `anyio`)
- **100%** [Type Completeness](https://microsoft.github.io/pyright/#/typed-libraries?id=type-completeness)
- Ergonomic API (and as close as possible to the Tauri Rust API)
    - Python

        ```python
        import sys

        from pydantic import BaseModel
        from pytauri import (
            AppHandle,
            Commands,
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
        ```

    - Frontend

        ```ts
        import { pyInvoke } from "tauri-plugin-pytauri-api";
        // or: `const { pyInvoke } = window.__TAURI__.pytauri;`

        export interface Person {
            name: string;
        }

        export interface Greeting {
            message: string;
        }

        export async function greet(body: Person): Promise<Greeting> {
            return await pyInvoke("greet", body);
        }
        ```

- Can be integrated with [nicegui]/[gradio]/[FastAPI] to achieve a full-stack Python development experience (i.g., without `Node.js`). See [examples/nicegui-app](https://github.com/WSH032/pytauri/tree/main/examples/nicegui-app).

## Release

| name | pypi | crates.io | npmjs |
|:-------:|:----:|:---------:|:-----:|
| 👉 **core** | - | - | - |
| pytauri | [![pytauri-pypi-v]][pytauri-pypi] | [![pytauri-crates-v]][pytauri-crates] | |
| pytauri-core | | [![pytauri-core-crates-v]][pytauri-core-crates] | |
| tauri-plugin-pytauri | | [![tauri-plugin-pytauri-crates-v]][tauri-plugin-pytauri-crates] | [![tauri-plugin-pytauri-api-npm-v]][tauri-plugin-pytauri-api-npm] |
| 👉 **plugins** | - | - | - |
| pytauri-plugin-notification | [![pytauri-plugin-notification-pypi-v]][pytauri-plugin-notification-pypi] | [![pytauri-plugin-notification-crates-v]][pytauri-plugin-notification-crates] | |
| 👉 **utils** | - | - | - |
| pyo3-utils | [![pyo3-utils-pypi-v]][pyo3-utils-pypi] | [![pyo3-utils-crates-v]][pyo3-utils-crates] | |
| codelldb | [![codelldb-pypi-v]][codelldb-pypi] | | |

[pytauri-pypi-v]: https://img.shields.io/pypi/v/pytauri
[pytauri-pypi]: https://pypi.org/project/pytauri
[pytauri-crates-v]: https://img.shields.io/crates/v/pytauri
[pytauri-crates]: https://crates.io/crates/pytauri
[pytauri-core-crates-v]: https://img.shields.io/crates/v/pytauri-core
[pytauri-core-crates]: https://crates.io/crates/pytauri-core
[tauri-plugin-pytauri-crates-v]: https://img.shields.io/crates/v/tauri-plugin-pytauri
[tauri-plugin-pytauri-crates]: https://crates.io/crates/tauri-plugin-pytauri
[tauri-plugin-pytauri-api-npm-v]:https://img.shields.io/npm/v/tauri-plugin-pytauri-api
[tauri-plugin-pytauri-api-npm]: https://www.npmjs.com/package/tauri-plugin-pytauri-api
[pytauri-plugin-notification-pypi-v]: https://img.shields.io/pypi/v/pytauri-plugin-notification
[pytauri-plugin-notification-pypi]: https://pypi.org/project/pytauri-plugin-notification
[pytauri-plugin-notification-crates-v]: https://img.shields.io/crates/v/pytauri-plugin-notification
[pytauri-plugin-notification-crates]: https://crates.io/crates/pytauri-plugin-notification
[pyo3-utils-pypi-v]: https://img.shields.io/pypi/v/pyo3-utils
[pyo3-utils-pypi]: https://pypi.org/project/pyo3-utils
[pyo3-utils-crates-v]: https://img.shields.io/crates/v/pyo3-utils
[pyo3-utils-crates]: https://crates.io/crates/pyo3-utils
[codelldb-pypi-v]: https://img.shields.io/pypi/v/codelldb
[codelldb-pypi]: https://pypi.org/project/codelldb

## Philosophy

### For Pythoneer

I hope `PyTauri` can become an alternative to [pywebview] and [Pystray], leveraging Tauri's comprehensive features to offer Python developers a GUI framework and a batteries-included development experience similar to [electron] and [PySide].

> PyTauri is inspired by [FastAPI] and [Pydantic], aiming to offer a similar development experience.

### For Rustacean

Through [Pyo3], I hope Rust developers can better utilize the Python ecosystem (e.g., building AI GUI applications with [PyTorch]).

Although Rust's lifetime and ownership system makes Rust code safer, Python's garbage collection (GC) will make life easier. 😆

[pywebview]: https://github.com/r0x0r/pywebview
[Pystray]: https://github.com/moses-palmer/pystray
[electron]: https://github.com/electron/electron
[PySide]: https://wiki.qt.io/Qt_for_Python
[FastAPI]: https://github.com/fastapi/fastapi
[Pydantic]: https://github.com/pydantic/pydantic
[PyTorch]: https://github.com/pytorch/pytorch
[nicegui]: https://github.com/zauberzeug/nicegui
[gradio]: https://github.com/gradio-app/gradio

## Credits

PyTauri is a project that aims to provide Python bindings for [Tauri], a cross-platform webview GUI library. `Tauri` is a trademark of the Tauri Program within the Commons Conservancy and PyTauri is not officially endorsed or supported by them. PyTauri is an independent and community-driven effort that respects the original goals and values of Tauri. PyTauri does not claim any ownership or affiliation with the Tauri Program.

## License

This project is licensed under the terms of the *Apache License 2.0*.
