// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{command, AppHandle, Runtime};

/// Check camera permission.
///
/// # Returns
/// - `bool`: `true` if camera permission is granted, `false` otherwise.
///
/// On non-OHOS platforms, always returns `true`.
#[command]
pub async fn check_camera_permission<R: Runtime>(app: AppHandle<R>) -> bool {
    #[cfg(any(mobile, target_env = "ohos"))]
    {
        use crate::OhosPermissionsExt;
        return app.ohos_permissions().check_permission("camera").await;
    }

    #[cfg(not(any(mobile, target_env = "ohos")))]
    {
        let _ = app;
        true
    }
}

/// Request camera permission.
///
/// Triggers the OHOS system permission dialog.
/// On non-OHOS platforms, this is a no-op.
#[command]
pub async fn request_camera_permission<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    #[cfg(any(mobile, target_env = "ohos"))]
    {
        use crate::OhosPermissionsExt;
        return app.ohos_permissions().request_permission("camera").await;
    }

    #[cfg(not(any(mobile, target_env = "ohos")))]
    {
        let _ = app;
        Ok(())
    }
}
