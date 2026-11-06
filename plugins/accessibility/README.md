# @tauri-apps/plugin-accessibility

Accessibility state queries (font scale, screen reader, touch exploration) for
Tauri applications on OpenHarmony.

> OpenHarmony-only. On other platforms the commands reject with `unsupported`.
> Web content accessibility is handled by ArkWeb's built-in ARIA support and is
> out of scope.

## Install

**Rust** (OpenHarmony target only):

```toml
[target.'cfg(target_env = "ohos")'.dependencies]
tauri-plugin-accessibility = { path = "..." }
```

```rust
#[cfg(target_env = "ohos")]
tauri::Builder::default().plugin(tauri_plugin_accessibility::init())
```

**JavaScript**:

```sh
npm add @tauri-apps/plugin-accessibility
```

## API

```ts
import {
  getFontScale,
  isScreenReaderEnabled,
  isTouchExploreEnabled,
  onAccessibilityStateChange
} from '@tauri-apps/plugin-accessibility'

const scale = await getFontScale() // e.g. 1.0 — no permission required
const screenReader = await isScreenReaderEnabled()
const touchExplore = await isTouchExploreEnabled()

const unlisten = await onAccessibilityStateChange((enabled) => {
  // screen reader was toggled
})
```

- `getFontScale()` reads the ability configuration — no permission required.
- `isScreenReaderEnabled()` / `isTouchExploreEnabled()`: OHOS documents the
  system-level `ohos.permission.ACCESSIBILITY` permission for these queries; a
  denial rejects with an error message carrying the BusinessError code rather
  than a silent `false`.
- `onAccessibilityStateChange` fans out through the Tauri event system
  (`accessibility-state-changed`, boolean payload); multiple listeners are
  supported. On other platforms the listener resolves but the event never fires.

## Permissions

Add `accessibility:default` to your application capabilities:

```json
{ "identifier": "accessibility:default" }
```

`accessibility:default` grants `allow-get-font-scale`,
`allow-is-screen-reader-enabled`, and `allow-is-touch-explore-enabled`. No
`module.json5` permission declarations are required.

## License

MIT OR Apache-2.0
