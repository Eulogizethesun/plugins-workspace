// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg(not(any(target_os = "android", target_os = "ios")))]

#[cfg(not(target_env = "ohos"))]
mod commands;
#[cfg(target_env = "ohos")]
mod ohos;

mod error;

pub use error::{Error, Result};

use serde::Serialize;

/// A webview screenshot returned to JS (base64 PNG + pixel dimensions).
///
/// The bridge facade's `CapturedImage` is a pure Rust type without serde; this DTO is
/// the wire shape (`camelCase` per plugin convention). Shared by the OHOS
/// implementation and the non-OHOS stubs so both platform branches expose the
/// same command signatures.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturedImageDto {
    pub png_base64: String,
    pub width: u32,
    pub height: u32,
}

/// A single pixel's color channels (0-255 each).
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RgbaDto {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};

#[derive(Default)]
pub struct Builder;

impl Builder {
    pub fn new() -> Self {
        Self
    }

    #[cfg(target_env = "ohos")]
    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        PluginBuilder::new("screenshot")
            .invoke_handler(tauri::generate_handler![
                ohos::capture_webview,
                ohos::pick_color,
            ])
            .setup(|_app, _api| {
                // No ArkTS plugin of our own and no state to subscribe: the
                // `capture-webview` / `pick-color` actions ride the globally-registered
                // WebviewBridgePlugin (tauri-runtime-wry registers it), so setup has
                // nothing to do beyond confirming the plugin loaded.
                log::info!("[screenshot] plugin initialized");
                Ok(())
            })
            .build()
    }

    #[cfg(not(target_env = "ohos"))]
    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        PluginBuilder::new("screenshot")
            .invoke_handler(tauri::generate_handler![
                commands::capture_webview,
                commands::pick_color,
            ])
            .build()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new().build()
}
