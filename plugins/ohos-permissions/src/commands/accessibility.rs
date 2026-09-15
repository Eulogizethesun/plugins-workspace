// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{command, AppHandle, Runtime};

/// Check accessibility permission.
///
/// OHOS `AccessibilityExtensionAbility` is deprecated since API 12 and
/// replacement APIs are system-only. Third-party apps cannot register
/// accessibility services. Always returns `false` on OHOS.
#[command]
pub async fn check_accessibility_permission<R: Runtime>(app: AppHandle<R>) -> bool {
    let _ = app;
    // OHOS stub: no equivalent to macOS application_is_trusted()
    false
}

/// Request accessibility permission.
///
/// OHOS has no equivalent to macOS `application_is_trusted_with_prompt()`.
/// This is a no-op.
#[command]
pub async fn request_accessibility_permission<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let _ = app;
    Ok(())
}
