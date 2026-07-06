// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{command, AppHandle, Runtime};

/// Check full disk access permission.
///
/// OHOS has a different sandbox model from macOS and does not have
/// an equivalent "full disk access" concept. Always returns `true`.
#[command]
pub async fn check_full_disk_access_permission<R: Runtime>(app: AppHandle<R>) -> bool {
    let _ = app;
    // OHOS stub: no equivalent concept, treat as always granted
    true
}

/// Request full disk access permission.
///
/// OHOS has no equivalent operation. This is a no-op.
#[command]
pub async fn request_full_disk_access_permission<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let _ = app;
    Ok(())
}
