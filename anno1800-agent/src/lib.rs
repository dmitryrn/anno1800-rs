#![allow(clippy::missing_safety_doc)]
use log::{debug, error, info, trace};
use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    thread::{self},
    time::Duration,
};
use windows::{
    imp::GetLastError,
    s,
    Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE},
};

use crate::hooks::{handle_loop_over_islands_hook, handle_update_potential_production_hook};

pub mod api;
pub mod ffi;
pub mod hooks;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
    debug!("start()");

    // For good measure
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0xd44c79, handle_update_potential_production_hook as usize).unwrap();

    // This does not cover all objects going into `loop_over_prod_buildings`, but some
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0x7B0F3C, handle_loop_over_islands_hook as usize).unwrap();
    info!("Anno1800 agent startup completed sucessfully.");
    1
}

#[no_mangle]
pub unsafe extern "C" fn stop() -> u32 {
    debug!("stop()");
    ffi::unhook_call_rel32(s!("Anno1800.exe"), 0xd44c79, 0x00009782).unwrap();
    ffi::unhook_call_rel32(s!("Anno1800.exe"), 0x7b0f2b, 0x004f9cd0).unwrap();
    info!("Stop completed sucessfully");
    1
}
