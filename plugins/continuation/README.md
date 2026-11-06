# @tauri-apps/plugin-continuation

App continuation (cross-device handoff) support for Tauri applications on
OpenHarmony: passive restore queries on the target device plus a source-side
snapshot API.

> OpenHarmony-only. On other platforms the commands reject with `unsupported`.
> Active migration (source device initiating a hand-off) is system-UI-exclusive on
> OpenHarmony and is not covered by this plugin.

## Install

**Rust** (OpenHarmony target only):

```toml
[target.'cfg(target_env = "ohos")'.dependencies]
tauri-plugin-continuation = { path = "..." }
```

```rust
#[cfg(target_env = "ohos")]
tauri::Builder::default().plugin(tauri_plugin_continuation::init())
```

**JavaScript**:

```sh
npm add @tauri-apps/plugin-continuation
```

## API

### Target device (restore)

```ts
import {
  isContinuationRestoreLaunch,
  getContinuationData
} from '@tauri-apps/plugin-continuation'

// Was this launch a continuation restore? Peek semantics — idempotent.
if (await isContinuationRestoreLaunch()) {
  // The full wantParam the system delivered, as a raw JSON string.
  // Consuming API — only the FIRST call returns the payload; later calls
  // (and non-continuation launches) return null.
  const payload = await getContinuationData()
  // Your source-side snapshot is nested under the reserved `continuationData` key.
  const { scrollOffset } = JSON.parse(JSON.parse(payload!).continuationData)
}
```

### Source device (snapshot)

```ts
import { setContinuationData } from '@tauri-apps/plugin-continuation'

// Call this while running, BEFORE the system initiates a migration — the
// onContinue callback reads the snapshot synchronously and never waits for JS.
// Max 96 KiB; '' clears the snapshot (the system then refuses the migration).
// Overwrite-on-set, peek-on-read: a cancelled migration leaves the snapshot
// intact for a retry.
await setContinuationData(JSON.stringify({ scrollOffset: window.scrollY }))
```

The snapshot is forwarded verbatim as `wantParam.continuationData`; the rest of
the delivered `wantParam` is system-injected. The payload schema is an
application-level contract — the plugin passes it through verbatim.

## Permissions

`continuation:default` grants `allow-is-continuation-restore`,
`allow-get-continuation-data`, and `allow-set-continuation-data`. No
`module.json5` permission declarations are required — the continuation signal is
captured from the ability lifecycle, not a system service.

## License

MIT OR Apache-2.0
