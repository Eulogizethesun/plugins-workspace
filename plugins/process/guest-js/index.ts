// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * Perform operations on the current process.
 * @module
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * Exits immediately with the given `exitCode`.
 *
 * #### Platform-specific
 *
 * - **HarmonyOS (OHOS):** The exit code is not propagated to the process
 *   exit status — the process always exits with code `0`. The requested code
 *   is still delivered to the Rust-side `RunEvent::ExitRequested` handler.
 *
 * @example
 * ```typescript
 * import { exit } from '@tauri-apps/plugin-process';
 * await exit(1);
 * ```
 *
 * @param code The exit code to use.
 * @returns A promise indicating the success or failure of the operation.
 *
 * @since 2.0.0
 */
async function exit(code = 0): Promise<void> {
  await invoke('plugin:process|exit', { code })
}

/**
 * Exits the current instance of the app then relaunches it.
 *
 * #### Platform-specific
 *
 * - **HarmonyOS (OHOS):** Relaunch is performed by the official
 *   `ApplicationContext.restartApp` (API 12+), which **requires the app to be
 *   in the foreground**: when called while the app is in the background the
 *   restart is rejected (error 16000053, logged by the system as
 *   "Not top ability") and the app **exits without relaunching**. The
 *   rejection happens after the process has committed to exiting, so it
 *   cannot be observed or handled from JS. Once the restart is accepted, the
 *   process parks waiting for the ability runtime to kill it (no timeout),
 *   and the ability's `onDestroy` is not fired.
 *
 * @example
 * ```typescript
 * import { relaunch } from '@tauri-apps/plugin-process';
 * await relaunch();
 * ```
 *
 * @returns A promise indicating the success or failure of the operation.
 *
 * @since 2.0.0
 */
async function relaunch(): Promise<void> {
  await invoke('plugin:process|restart')
}

export { exit, relaunch }
