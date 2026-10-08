// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{command, AppHandle, Runtime};

/// Check screen recording permission.
///
/// OHOS `ohos.permission.CAPTURE_SCREEN` is `system_basic` level,
/// unavailable to third-party apps. Always returns `false`.
#[command]
pub async fn check_screen_recording_permission<R: Runtime>(app: AppHandle<R>) -> bool {
    let _ = app;
    false
}

/// Request screen recording permission.
///
/// OHOS `ohos.permission.CAPTURE_SCREEN` is `system_basic` level,
/// unavailable to third-party apps. This is a no-op.
#[command]
pub async fn request_screen_recording_permission<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let _ = app;
    Ok(())
}
