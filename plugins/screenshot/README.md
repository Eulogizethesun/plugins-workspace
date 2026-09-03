# @tauri-apps/plugin-screenshot

In-app webview screenshot and color picking for Tauri applications on OpenHarmony.

> OpenHarmony-only. On other platforms the commands reject with `unsupported`.

## Install

**Rust** (OpenHarmony target only):

```toml
[target.'cfg(target_env = "ohos")'.dependencies]
tauri-plugin-screenshot = { path = "..." }
```

```rust
#[cfg(target_env = "ohos")]
tauri::Builder::default().plugin(tauri_plugin_screenshot::init())
```

**JavaScript**:

```sh
npm add @tauri-apps/plugin-screenshot
```

## API

```ts
import { captureWebview, pickColorAt } from '@tauri-apps/plugin-screenshot'

// Full-viewport snapshot of the calling webview.
const image = await captureWebview()
// image.pngBase64 — base64 PNG
// image.width / image.height — pixel dimensions

// Read one pixel (snapshot coordinate system — use the dimensions returned by
// captureWebview to scale CSS coordinates).
const { r, g, b, a } = await pickColorAt(x, y)
```

## Permissions

`screenshot:default` grants `allow-capture-webview` and `allow-pick-color`. No
`module.json5` permission declarations are required — the snapshot is taken from the
app's own webview (ArkWeb `webPageSnapshot`), not the screen.

## Errors

- `unknown webview` — the calling webview is not registered (e.g. torn down).
- `snapshot unavailable` — the snapshot could not be produced (retries exhausted,
  timeout, or the webview has not rendered its first frame).
- anything else — packing / pixel-read / coordinate failure, message included.

## Platform notes

- The snapshot covers the calling webview's **viewport** (the on-screen area), not
  the full scrollable page — the snapshot origin matches the viewport origin.
- Transparent (`SYNC_RENDER`) webviews are **unverified**: ArkWeb
  `webPageSnapshot` behavior under `RenderMode.SYNC_RENDER` has not been
  validated on device, so a capture from a transparent window may fail with
  `snapshot unavailable`.
- `captureWebview` returns the PNG as a base64 string inside the response object
  — large windows produce multi-hundred-KB payloads (measured ~500 KB at
  2092×1249) that cross the bridge in a single call; the bridge's payload cap
  only applies to bare string payloads, not object fields.
- `pickColorAt` takes a full webview snapshot on every call (~1 s round-trip on
  device); avoid calling it in tight loops.

## License

MIT OR Apache-2.0
