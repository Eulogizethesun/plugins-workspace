// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

const COMMANDS: &[&str] = &[
    "initialize",
    "destroy",
    "save",
    "create_client",
    "load_client",
    "get_store_record",
    "save_store_record",
    "remove_store_record",
    "save_secret",
    "remove_secret",
    "execute_procedure",
];

fn main() {
    // OHOS cross-compilation (host is not OHOS): require prebuilt libsodium
    // via SODIUM_LIB_DIR — libsodium-sys-stable's ./configure cannot run on
    // this host. On an OHOS PC (host == target) the automatic source build
    // works without any setup (README "OHOS Build"). CARGO_CFG_TARGET_ENV
    // reflects the TARGET triple; cfg!(target_env = "ohos") in build.rs
    // reflects the HOST — combining them detects cross-compilation.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("ohos")
        && !cfg!(target_env = "ohos")
    {
        println!("cargo:rerun-if-env-changed=SODIUM_LIB_DIR");
        match std::env::var("SODIUM_LIB_DIR") {
            Ok(dir) => {
                println!("cargo:warning=[stronghold] OHOS: using prebuilt libsodium from {dir}");
            }
            Err(_) => {
                let target = std::env::var("TARGET").unwrap_or_else(|_| "the OHOS target".to_string());
                panic!(
                    "cross-compiling to OHOS requires SODIUM_LIB_DIR to be set. \
                     libsodium-sys-stable's build script invokes ./configure, which cannot \
                     produce {target} artifacts on this host (on a Windows host it fails \
                     outright with os error 193; on a Unix host it builds for the host \
                     architecture instead). Its build script runs in a separate process \
                     and cannot read env vars set here, so the variable must be set in \
                     the environment invoking cargo. Obtain a prebuilt libsodium for \
                     {target} (from the OHOS PC Conan registry / cmd-pkgs, or built once \
                     with the OHOS NDK clang) and point SODIUM_LIB_DIR at its lib \
                     directory. See README.md OHOS Build."
                );
            }
        }
    }

    tauri_plugin::Builder::new(COMMANDS)
        .global_api_script_path("./api-iife.js")
        .build();
}
