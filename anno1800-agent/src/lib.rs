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

use crate::ffi::handle_update_potential_production_hook;
mod api;
mod ffi;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
    debug!("start()");
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0xd44c79, handle_update_potential_production_hook as usize).unwrap();
    info!("Anno1800 agent startup completed sucessfully.");
    1
}

#[no_mangle]
pub unsafe extern "C" fn stop() -> u32 {
    debug!("stop()");
    ffi::unhook_call_rel32(s!("Anno1800.exe"), 0xd44c79, 0x00009782).unwrap();
    info!("Stop completed sucessfully");
    1
}
