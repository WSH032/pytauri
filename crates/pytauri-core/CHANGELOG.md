# pytauri-core

## [Unreleased]

## [0.2.0]

### BREAKING

- [#57](https://github.com/WSH032/pytauri/pull/57) - refactor: remove `RunEventEnum`, use matched `RunEvent` directly.
- [#56](https://github.com/WSH032/pytauri/pull/56) - perf: `Invoke::bind_to` now returns `[Self::BODY_KEY]`: `PyBytes` instead of `PyByteArray`.

### Added

- [#62](https://github.com/WSH032/pytauri/pull/62) - feat: add `path::PathResolver`, `Manager::path`.
- [#50](https://github.com/WSH032/pytauri/pull/50) - feat: add `ipc::Channel`, `ipc::JavaScriptChannelId`, `webview::Webview`, `webview::WebviewWindow::as_ref::<webview>` for [channels ipc](https://tauri.app/develop/calling-frontend/#channels).
- [#46](https://github.com/WSH032/pytauri/pull/46) - feat: add `webview::WebviewWindow`, `Manager`, `ImplManager`, `App::handle`.
- [#48](https://github.com/WSH032/pytauri/pull/48) - feat: accessing the `WebviewWindow` in `Commands`.
- [#49](https://github.com/WSH032/pytauri/pull/49) - feat: add `Event`, `EventId`, `Listener`, `ImplListener` for [Event System](https://tauri.app/develop/calling-frontend/#event-system).

## [0.1.0-beta.1]

## [0.1.0-beta.0]

[unreleased]: https://github.com/WSH032/pytauri/tree/HEAD
[0.2.0]: https://github.com/WSH032/pytauri/releases/tag/rs/pytauri-core/v0.2.0
[0.1.0-beta.1]: https://github.com/WSH032/pytauri/releases/tag/rs/pytauri-core/v0.1.0-beta.1
[0.1.0-beta.0]: https://github.com/WSH032/pytauri/releases/tag/rs/pytauri-core/v0.1.0-beta.0
