// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * Check and request OHOS system permissions.
 *
 * @module
 */

import { invoke } from '@tauri-apps/api/core'

// ─── Accessibility (stub: OHOS third-party apps cannot register accessibility services) ───

/**
 * Check accessibility permission.
 *
 * OHOS `AccessibilityExtensionAbility` is deprecated since API 12.
 * Always returns `false` for third-party apps.
 */
export async function checkAccessibilityPermission(): Promise<boolean> {
  return invoke<boolean>('plugin:ohos-permissions|check_accessibility_permission')
}

/**
 * Request accessibility permission.
 *
 * OHOS has no equivalent API. This is a no-op.
 */
export async function requestAccessibilityPermission(): Promise<void> {
  await invoke('plugin:ohos-permissions|request_accessibility_permission')
}

// ─── Camera ───

export async function checkCameraPermission(): Promise<boolean> {
  return invoke<boolean>('plugin:ohos-permissions|check_camera_permission')
}

export async function requestCameraPermission(): Promise<void> {
  await invoke('plugin:ohos-permissions|request_camera_permission')
}

// ─── Microphone ───

export async function checkMicrophonePermission(): Promise<boolean> {
  return invoke<boolean>('plugin:ohos-permissions|check_microphone_permission')
}

export async function requestMicrophonePermission(): Promise<void> {
  await invoke('plugin:ohos-permissions|request_microphone_permission')
}

// ─── Full Disk Access (stub: OHOS has no equivalent) ───

/**
 * Check full disk access permission.
 *
 * OHOS has a different sandbox model and no equivalent concept.
 * Always returns `true`.
 */
export async function checkFullDiskAccessPermission(): Promise<boolean> {
  return invoke<boolean>('plugin:ohos-permissions|check_full_disk_access_permission')
}

/**
 * Request full disk access permission.
 *
 * OHOS has no equivalent operation. This is a no-op.
 */
export async function requestFullDiskAccessPermission(): Promise<void> {
  await invoke('plugin:ohos-permissions|request_full_disk_access_permission')
}

// ─── Screen Recording (stub: system_basic, unavailable to third-party apps) ───

/**
 * Check screen recording permission.
 *
 * OHOS `ohos.permission.CAPTURE_SCREEN` is `system_basic` level.
 * Always returns `false` for third-party apps.
 */
export async function checkScreenRecordingPermission(): Promise<boolean> {
  return invoke<boolean>('plugin:ohos-permissions|check_screen_recording_permission')
}

/**
 * Request screen recording permission.
 *
 * OHOS `ohos.permission.CAPTURE_SCREEN` is `system_basic` level.
 * This is a no-op for third-party apps.
 */
export async function requestScreenRecordingPermission(): Promise<void> {
  await invoke('plugin:ohos-permissions|request_screen_recording_permission')
}

// ─── Input Monitoring (stub: system_basic, unavailable to third-party apps) ───

/**
 * Check input monitoring permission.
 *
 * OHOS `ohos.permission.INPUT_MONITORING` is `system_basic` level.
 * Always returns `false` for third-party apps.
 */
export async function checkInputMonitoringPermission(): Promise<boolean> {
  return invoke<boolean>('plugin:ohos-permissions|check_input_monitoring_permission')
}

/**
 * Request input monitoring permission.
 *
 * OHOS `ohos.permission.INPUT_MONITORING` is `system_basic` level.
 * This is a no-op for third-party apps.
 */
export async function requestInputMonitoringPermission(): Promise<void> {
  await invoke('plugin:ohos-permissions|request_input_monitoring_permission')
}
