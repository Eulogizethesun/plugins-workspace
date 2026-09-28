![plugin-clipboard-manager](https://github.com/tauri-apps/plugins-workspace/raw/v2/plugins/clipboard-manager/banner.png)

Read and write to the system clipboard.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓         |
| iOS      | ✓         |
| OpenHarmony | ✓ (reads require a permission, see below) |

## Install

_This plugin requires a Rust version of at least **1.77.2**_

There are three general methods of installation that we can recommend.

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from Github using git tags / revision hashes (most secure)
3. Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-clipboard-manager = "2.0.0"
# alternatively with Git:
tauri-plugin-clipboard-manager = { git = "https://github.com/tauri-apps/plugins-workspace", branch = "v2" }
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```sh
pnpm add @tauri-apps/plugin-clipboard-manager
# or
npm add @tauri-apps/plugin-clipboard-manager
# or
yarn add @tauri-apps/plugin-clipboard-manager
```

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```javascript
import {
  writeText,
  readText,
  writeHtml,
  clear
} from '@tauri-apps/plugin-clipboard-manager'
await writeText('Tauri is awesome!')
assert(await readText(), 'Tauri is awesome!')
```

## OpenHarmony (OHOS) Permissions

On HarmonyOS the clipboard **read** APIs (`readText`, `readImage`) are governed
by a pasteboard read permission since API 12; the **write** APIs (`writeText`,
`writeImage`, `writeHtml`) and `clear` need no permission.

### `ohos.permission.READ_PASTEBOARD`

- **Level / grant mode:** restricted permission — `system_basic` level,
  authorized per user (`user_grant`), available since API 11.
- **`module.json5` declaration:** must be declared in the app's entry
  `module.json5` with `reason` and `usedScene`:

  ```json
  {
    "name": "ohos.permission.READ_PASTEBOARD",
    "reason": "$string:reason_read_pasteboard",
    "usedScene": {
      "abilities": ["EntryAbility"],
      "when": "inuse"
    }
  }
  ```

  Projects created with the tauri-cli open-harmony template already include
  this declaration and the `reason_read_pasteboard` string resource; older
  projects or custom `module.json5` files need to add them manually.

- **Signing profile (ACL):** as a restricted permission it must also be
  requested through AppGallery Connect and present in the signing profile's
  ACL. Without the ACL entry the app fails to sign/install — a hard failure at
  install time, not a runtime degradation. Apps on PC and 2-in-1 devices can
  all apply for it (AGC review is expected to take about 3 working days; the
  first runtime request is granted by default without a dialog, and the user
  can only switch between allow/forbid in Settings). On other devices only
  whitelist scenarios (bank card numbers, passwords, document editing,
  system-level input methods, open-source frameworks) pass the review.

- **When the permission is undeclared or denied:** the plugin degrades
  gracefully instead of surfacing a permission error —
  - `readText` resolves with an empty string `""`,
  - `readImage` rejects with a "clipboard does not contain an image" error.

  Both are indistinguishable from an actually empty clipboard.

See the official
[pasteboard read permission guidelines](https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/get-pastedata-permission-guidelines)
for details.

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

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
