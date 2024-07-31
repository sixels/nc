// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use std::sync::atomic::{AtomicU32, Ordering};

static SEG_COUNT: AtomicU32 = AtomicU32::new(0);

fn handle_segfault(sig: i32) {
    eprintln!("Got segfault");
    assert_eq!(sig, nc::SIGSEGV);

    let count = SEG_COUNT.fetch_add(1, Ordering::Relaxed);
    if count >= 3 {
        eprintln!("Too many segfault, exit now");
        unsafe {
            nc::exit(2);
        }
    }
}

#[must_use]
#[inline]
fn get_sa_restorer() -> Option<nc::restorefn_t> {
    let mut old_sa = nc::sigaction_t::default();
    let ret = unsafe { nc::rt_sigaction(nc::SIGSEGV, None, Some(&mut old_sa)) };
    if ret.is_ok() {
        old_sa.sa_restorer
    } else {
        None
    }
}

fn main() {
    // Register SIGSEGV handler.
    let sa = nc::sigaction_t {
        sa_handler: handle_segfault as nc::sighandler_t,
        sa_flags: nc::SA_RESTART | nc::SA_RESTORER,
        sa_restorer: get_sa_restorer(),
        ..nc::sigaction_t::default()
    };
    let ret = unsafe { nc::rt_sigaction(nc::SIGSEGV, Some(&sa), None) };
    assert!(ret.is_ok());

    // Initialize an anonymous mapping with 4 pages.
    let map_length = 4 * nc::PAGE_SIZE;
    #[cfg(target_arch = "arm")]
    let addr = unsafe {
        nc::mmap2(
            0,
            map_length,
            nc::PROT_READ | nc::PROT_WRITE,
            nc::MAP_PRIVATE | nc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };

    #[cfg(not(target_arch = "arm"))]
    let addr = unsafe {
        nc::mmap(
            0,
            map_length,
            nc::PROT_READ | nc::PROT_WRITE,
            nc::MAP_PRIVATE | nc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };
    assert!(addr.is_ok());
    let addr = addr.unwrap();

    // Set the third page readonly. And we will run into SIGSEGV when updating it.
    let ret = unsafe { nc::mprotect(addr + 2 * nc::PAGE_SIZE, nc::PAGE_SIZE, nc::PROT_READ) };
    assert!(ret.is_ok());

    for ptr in addr..(addr + map_length) {
        println!("access address: 0x{ptr:x}");

        // Trigger segfault
        unsafe {
            *(ptr as *mut u8) = 42;
        }
    }

    let ret = unsafe { nc::munmap(addr, map_length) };
    assert!(ret.is_ok());
    unsafe { nc::exit(0) };
}
