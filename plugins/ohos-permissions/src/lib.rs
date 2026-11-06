// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Check and request OHOS system permissions in Tauri applications.
//!
//! Supports Camera and Microphone permissions on OpenHarmony.
//! On non-OHOS platforms, all checks return `true` and requests are no-ops.

#![doc(
    html_logo_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png",
    html_favicon_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png"
)]

#[cfg(any(mobile, target_env = "ohos"))]
#[allow(unused_imports)]
use tauri::plugin::PluginHandle;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod commands;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(any(mobile, target_env = "ohos"))]
mod mobile;

#[cfg(any(mobile, target_env = "ohos"))]
pub use mobile::OhosPermissions;

pub use commands::*;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`], [`tauri::WebviewWindow`],
/// [`tauri::Webview`] and [`tauri::Window`] to access the OHOS permissions APIs.
pub trait OhosPermissionsExt<R: Runtime> {
    fn ohos_permissions(&self) -> &OhosPermissions<R>;
}

#[cfg(any(mobile, target_env = "ohos"))]
impl<R: Runtime, T: Manager<R>> OhosPermissionsExt<R> for T {
    fn ohos_permissions(&self) -> &OhosPermissions<R> {
        self.state::<OhosPermissions<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ohos-permissions")
        .invoke_handler(tauri::generate_handler![
            commands::check_accessibility_permission,
            commands::request_accessibility_permission,
            commands::check_camera_permission,
            commands::request_camera_permission,
            commands::check_microphone_permission,
            commands::request_microphone_permission,
            commands::check_full_disk_access_permission,
            commands::request_full_disk_access_permission,
            commands::check_screen_recording_permission,
            commands::request_screen_recording_permission,
            commands::check_input_monitoring_permission,
            commands::request_input_monitoring_permission,
        ])
        .setup(|app, api| {
            #[cfg(any(mobile, target_env = "ohos"))]
            let permissions = mobile::init(app, api)?;
            #[cfg(not(any(mobile, target_env = "ohos")))]
            let permissions = ();
            #[cfg(any(mobile, target_env = "ohos"))]
            app.manage(permissions);
            let _ = app;
            Ok(())
        })
        .build()
}
