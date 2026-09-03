// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{error::Error, CapturedImageDto, Result, RgbaDto};

#[tauri::command]
pub async fn capture_webview() -> Result<CapturedImageDto> {
    Err(Error::Unsupported)
}

#[tauri::command]
pub async fn pick_color(_x: u32, _y: u32) -> Result<RgbaDto> {
    Err(Error::Unsupported)
}
