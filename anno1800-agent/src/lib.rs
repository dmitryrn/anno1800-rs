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
    Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE},
};

use crate::ffi::handle_update_potential_production_hook;

mod ffi;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
    debug!("start()");

    ffi::hook_call_rel32(0x00007FF6F2374C79, handle_update_potential_production_hook as usize).unwrap(); // TODO aslr -.-

    info!("Anno1800 agent startup completed sucessfully.");
    1
}

#[no_mangle]
pub unsafe extern "C" fn stop() -> u32 {
    debug!("stop()");
    info!("Stop completed sucessfully");
    1
}
