![NFC](https://github.com/tauri-apps/plugins-workspace/raw/v2/plugins/nfc/banner.png)

Read and write NFC tags on Android, iOS and OpenHarmony.

| Platform | Supported |
| -------- | --------- |
| Linux    | x         |
| Windows  | x         |
| macOS    | x         |
| Android  | ✓         |
| iOS      | ✓         |
| OpenHarmony | ✓ (partial, see below) |

## Install

_This plugin requires a Rust version of at least **1.65**_

There are three general methods of installation that we can recommend.

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from Github using git tags / revision hashes (most secure)
3. Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-nfc = "2.0.0"
# alternatively with Git:
tauri-plugin-nfc = { git = "https://github.com/tauri-apps/plugins-workspace", branch = "v2" }
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

<!-- Add the branch for installations using git! -->

```sh
pnpm add @tauri-apps/plugin-nfc
# or
npm add @tauri-apps/plugin-nfc
# or
yarn add @tauri-apps/plugin-nfc
```

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_nfc::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```javascript
import { scan, textRecord, write } from '@tauri-apps/plugin-nfc'
await scan({ type: 'tag', keepSessionAlive: true })
await write([textRecord('Tauri is awesome!')])
```

### OpenHarmony

`scan`/`write` (NDEF) are implemented through the NFC foreground reader mode
(`tag.on('readerMode')`, API 11+): the invoke stays pending until a tag is
tapped, mirroring the Android `Session` semantics. The required
`ohos.permission.NFC_TAG` (normal, system-granted) is declared by the plugin's
OpenHarmony HAR module and granted App-wide, so consuming apps need no extra
declaration. Platform differences to keep in mind:

- `mimeType`/`uri` scan filters have no readerMode equivalent and are silently
  ignored (discovery is RF-technology based; `techLists` are applied as a
  post-filter).
- The `kind` array in scan results holds OHOS short technology names (e.g.
  `NfcA`), while Android sends fully-qualified class names (e.g.
  `android.nfc.tech.NfcA`) — `kind` values are not directly comparable across
  platforms.
- Registration lifecycle: the system manages the readerMode registration per
  foreground session (official guidance pairs `tag.on('readerMode')` in
  `onForeground` with `tag.off` before leaving the foreground). The plugin has
  no `onBackground` hook, so it re-establishes the registration with a full
  off/on cycle whenever the app returns to the foreground with a session
  still armed — but an armed `scan()`/`write()` has no timeout and no cancel
  command: if the app is killed while armed, the JS promise dies with the
  process. Don't arm sessions across expected backgrounding.

The Rust-side API (`Nfc::scan()`/`Nfc::write()`) is currently not usable on
OpenHarmony — see the `openharmony` entry in the crate's
`[package.metadata.platforms.support]` notes — so use the JavaScript plugin
API.

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## Contributed By

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://crabnebula.dev" target="_blank">
          <img src="contributors/crabnebula.svg" alt="CrabNebula" width="283">
        </a>
      </td>
      <td align="center" valign="middle">
        <a href="https://impierce.com" target="_blank">
            <img src="contributors/impierce.svg" alt="Impierce" width="283" height="90">
        </a>
      </td>
    </tr>
  </tbody>
</table>

## Partners

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://crabnebula.dev" target="_blank">
          <img src="https://github.com/tauri-apps/plugins-workspace/raw/v2/.github/sponsors/crabnebula.svg" alt="CrabNebula" width="283">
        </a>
      </td>
    </tr>
  </tbody>
</table>

For the complete list of sponsors please visit our [website](https://tauri.app#sponsors) and [Open Collective](https://opencollective.com/tauri).

## License

Code: (c) 2015 - Present - The Tauri Programme within The Commons Conservancy.

MIT or MIT/Apache 2.0 where applicable.
