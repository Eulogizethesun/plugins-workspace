// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! This plugin provides APIs to access the current process. To spawn child processes, see the [`shell`](https://github.com/tauri-apps/tauri-plugin-shell) plugin.

#![doc(
    html_logo_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png",
    html_favicon_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png"
)]

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;

#[cfg(target_env = "ohos")]
mod ohos;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("process")
        .invoke_handler(tauri::generate_handler![
            commands::exit,
            #[cfg(target_env = "ohos")]
            ohos::restart,
            #[cfg(not(target_env = "ohos"))]
            commands::restart,
        ])
        .setup(|_app, _api| {
            // OHOS: register the Rust-side bridge plugin (ProcessBridgePlugin,
            // id "ohos.process") BEFORE any restart command can dispatch
            // through it, so the declaration flows to ArkTS and the
            // ProcessPlugin factory is selected during configurePlugins.
            // Mirrors the updater plugin's setup. APP-None is unreachable
            // (setup runs after mobile_entry_point! installs the app);
            // register_plugin is non-idempotent (Err on duplicate) so failures
            // are log-only, never propagate.
            #[cfg(target_env = "ohos")]
            {
                use openharmony_ability::ProcessBridgePlugin;
                if let Ok(guard) = tauri::ohos::APP.lock() {
                    if let Some(ohos_app) = guard.as_ref() {
                        if let Err(e) = ohos_app.register_plugin(ProcessBridgePlugin) {
                            log::error!("[process] failed to register ProcessBridgePlugin: {e}");
                        }
                    }
                }
            }
            Ok(())
        })
        .build()
}
