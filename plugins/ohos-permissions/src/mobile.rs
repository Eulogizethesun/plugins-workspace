// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_env = "ohos")]
const PLUGIN_IDENTIFIER: &str = "@tauri/plugin-ohos-permissions";

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "app.tauri.ohos_permissions";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_ohos_permissions);

// Initializes the platform plugin class
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<OhosPermissions<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "OhosPermissionsPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_ohos_permissions)?;
    #[cfg(target_env = "ohos")]
    let handle = api.register_ohos_plugin(PLUGIN_IDENTIFIER, "OhosPermissionsPlugin")?;
    Ok(OhosPermissions(handle))
}

/// Access to the OHOS permissions APIs.
///
/// You can get an instance via [`OhosPermissionsExt`](crate::OhosPermissionsExt).
pub struct OhosPermissions<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> OhosPermissions<R> {
    /// Check if a permission is granted.
    ///
    /// `permission_name` is a logical name: `"camera"` or `"microphone"`.
    /// The ArkTS handler maps this to the OHOS permission string.
    pub async fn check_permission(&self, permission_name: &str) -> bool {
        let method = format!("check{}Permission", capitalize(permission_name));
        let mut args = HashMap::new();
        args.insert("permissionName", permission_name);
        self.0
            .run_mobile_plugin::<bool>(&method, args)
            .unwrap_or(false)
    }

    /// Request a permission from the user.
    ///
    /// `permission_name` is a logical name: `"camera"` or `"microphone"`.
    pub async fn request_permission(&self, permission_name: &str) -> Result<(), String> {
        let method = format!("request{}Permission", capitalize(permission_name));
        let mut args = HashMap::new();
        args.insert("permissionName", permission_name);
        self.0
            .run_mobile_plugin::<()>(&method, args)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
