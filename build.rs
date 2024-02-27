// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn get_page_size() {
    let output = Command::new("getconf")
        .args(["PAGE_SIZE"])
        .output()
        .expect("Failed to run getconf");
    let page_size_val = std::str::from_utf8(&output.stdout).unwrap();
    let page_size_val = page_size_val.trim();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("page_size.rs");
    fs::write(
        dest_path,
        format!("pub const PAGE_SIZE: usize = {};", page_size_val),
    )
    .unwrap();
}

fn main() {
    let rustc_toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap_or_else(|_| "stable".to_owned());
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "x86_64".to_owned());
    // esi and ebp registers are not allowed in inline asm in x86/i686.
    if (rustc_toolchain.starts_with("nightly") && target_arch != "x86")
        || target_arch == "aarch64"
        || target_arch == "arm"
        || target_arch == "x86_64"
    {
        println!("cargo:rustc-cfg=has_asm");
    } else {
        let syscall_file = format!("src/syscalls/syscall_{}.c", target_arch);
        eprintln!("syscall file: {syscall_file}");
        cc::Build::new().file(syscall_file).compile("syscall");
    }
    get_page_size();
}
