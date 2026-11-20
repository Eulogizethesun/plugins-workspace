// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! OpenHarmony-specific implementation for the updater plugin.
//!
//! On OHOS, updates are hosted by AppGallery — the desktop updater API
//! (endpoints/pubkey from `tauri.conf.json`) does not apply. Instead of
//! faking the desktop commands, this module exposes:
//! - the cross-platform `check` command, backed by the AppGallery query —
//!   it honors the desktop contract's query half (`null` when no update is
//!   available), while the download half stays AppGallery-owned
//! - `check_app_gallery_update` queries `updateManager.checkAppUpdate()`
//!   (pure metadata, no dialog)
//! - `show_app_gallery_update_dialog` triggers `updateManager.showUpdateDialog()`
//!   (system dialog driving the full download + install flow)

use crate::{Error, Result};
use openharmony_ability::Updater;
use serde::Serialize;
use tauri::{Manager, Runtime, Webview};

// ── Types ────────────────────────────────────────────────────────────

/// Result of an AppGallery update check, serialized as
/// `{ available, currentVersion, version }`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AppGalleryUpdateInfo {
    available: bool,
    current_version: String,
    version: Option<String>,
}

/// Wire shape of the desktop `UpdateMetadata` (see guest-js) returned by
/// `check` when an update is available. `rid` is inert on OHOS — the
/// desktop download flow does not apply; the update is driven by the
/// system dialog (`show_app_gallery_update_dialog`).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateMetadata {
    rid: u32,
    current_version: String,
    version: String,
    date: Option<String>,
    body: Option<String>,
    raw_json: serde_json::Value,
}

// ── Helpers ──────────────────────────────────────────────────────────

/// Get the AppGallery updater bridge handle.
///
/// The `tauri::ohos::APP` guard is acquired and dropped inside this helper,
/// so the lock is never held across an await.
fn updater_bridge() -> Result<Updater> {
    let guard = tauri::ohos::APP
        .lock()
        .map_err(|_| Error::Network("OHOS APP mutex poisoned".into()))?;
    let app = guard
        .as_ref()
        .ok_or_else(|| Error::Network("OHOS APP not initialized".into()))?;
    app.updater()
        .map_err(|e| Error::Network(e.reason.to_string()))
}

// ── Commands ─────────────────────────────────────────────────────────

/// Cross-platform `check` command, AppGallery-backed on OHOS.
///
/// Honors the query half of the desktop contract: resolves to `null` when
/// no update is available, or to the update's metadata. The desktop
/// `CheckOptions` (headers/timeout/proxy/target/allowDowngrades) do not
/// apply — AppGallery is the update source — and are ignored, like the
/// options of the two OHOS-only commands below. The download half of the
/// desktop contract is NOT honored: the returned `rid` is inert, and the
/// update flow is user-driven via `show_app_gallery_update_dialog`.
///
/// Without this registration the JS `check()` call has no Rust handler, so
/// it falls through to the ArkTS plugin layer — which has no `updater`
/// module — and dies as "Plugin not found: updater".
#[tauri::command]
pub(crate) async fn check<R: Runtime>(
    _webview: Webview<R>,
) -> Result<Option<UpdateMetadata>> {
    let updater = updater_bridge()?;

    match updater.check().await {
        // No update available: AppGallery reports nothing, so `check()`
        // resolves to `null` — the desktop contract's no-update semantics.
        Ok(None) => Ok(None),
        Ok(Some(r)) => Ok(Some(UpdateMetadata {
            rid: 0,
            version: r.version.filter(|v| !v.is_empty()).unwrap_or_else(|| "unknown".into()),
            raw_json: serde_json::json!({
                "source": "appGallery",
                "currentVersion": r.current_version,
            }),
            current_version: r.current_version,
            date: None,
            body: None,
        })),
        Err(e) => Err(Error::Network(e.reason.to_string())),
    }
}

/// Check for updates via AppGallery (OHOS only).
///
/// This is a pure query — no dialog is shown. It takes no options: AppGallery
/// is the update source, so the desktop `check` arguments
/// (headers/timeout/proxy/target/allowDowngrades) do not apply.
#[tauri::command]
pub(crate) async fn check_app_gallery_update<R: Runtime>(
    webview: Webview<R>,
) -> Result<AppGalleryUpdateInfo> {
    let updater = updater_bridge()?;

    match updater.check().await {
        Ok(None) => {
            // No update available: AppGallery reports nothing, so fall back to
            // the bundled version from `Manager::package_info()`.
            Ok(AppGalleryUpdateInfo {
                available: false,
                current_version: webview.package_info().version.to_string(),
                version: None,
            })
        }
        Ok(Some(r)) => Ok(AppGalleryUpdateInfo {
            available: true,
            current_version: r.current_version,
            // The SDK 12 ArkTS fallback is the literal "unknown" (on API < 20
            // the new version number is not available); map that and the empty
            // string to `None` so JS can rely on `version == null`. Since
            // openharmony-ability#51 the bridge itself returns `Option<String>`
            // (null on API < 20), which is passed through here.
            version: r.version.filter(|v| !v.is_empty() && v != "unknown"),
        }),
        Err(e) => Err(Error::Network(e.reason.to_string())),
    }
}

/// Show the system AppGallery update dialog (OHOS only).
///
/// The dialog is user-driven: whether to download and install the update is
/// decided by the user, so resolving `Ok(())` does NOT mean the update was
/// installed.
#[tauri::command]
pub(crate) async fn show_app_gallery_update_dialog<R: Runtime>(
    _webview: Webview<R>,
) -> Result<()> {
    updater_bridge()?
        .download_and_install()
        .await
        .map_err(|e| Error::Network(e.reason.to_string()))?;

    Ok(())
}
