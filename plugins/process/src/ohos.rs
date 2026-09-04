// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! OpenHarmony-specific implementation for the process plugin.

use std::time::Duration;

use openharmony_ability::Process;
use tauri::{AppHandle, Runtime};

/// Acquire the process bridge handle without holding the `APP` mutex across
/// an await point (a `MutexGuard` in the async command's future would make it
/// non-`Send`).
fn bridge_process() -> Option<Process> {
    let guard = match tauri::ohos::APP.lock() {
        Ok(guard) => guard,
        Err(_) => {
            log::error!("OHOS APP mutex poisoned — cannot restart");
            return None;
        }
    };
    let app = match guard.as_ref() {
        Some(app) => app,
        None => {
            log::error!("OHOS APP not initialized — cannot restart");
            return None;
        }
    };
    match app.process() {
        Ok(process) => Some(process),
        Err(e) => {
            log::error!("OHOS restart bridge unavailable: {e}");
            None
        }
    }
}

/// Restart the app on OpenHarmony by calling `appRecovery.restartApp()`
/// through the `ohos.process` bridge plugin (openharmony-ability
/// `crates/ability/src/process.rs`), bypassing the tao event loop (which does
/// not reliably deliver `RequestExit` on OHOS).
///
/// The process is hard-killed by `restartApp` — `onDestroy` is NOT triggered.
/// After dispatching `restartApp` to the main thread, this command blocks
/// forever, same pattern as the non-OHOS restart path (let the runtime
/// terminate us).
#[tauri::command]
pub async fn restart<R: Runtime>(_app: AppHandle<R>) {
    let dispatched = match bridge_process() {
        Some(process) => match process.restart().await {
            Ok(0) => true,
            Ok(code) => {
                log::error!("OHOS restart returned non-zero code: {code}");
                false
            }
            Err(e) => {
                log::error!("OHOS restart failed: {e}");
                false
            }
        },
        None => false,
    };
    if dispatched {
        // restartApp dispatched to main thread; block and let it kill the process
        loop {
            std::thread::sleep(Duration::MAX);
        }
    }
    std::process::exit(0);
}
