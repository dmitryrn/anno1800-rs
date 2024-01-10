#![allow(clippy::missing_safety_doc)]
use log::{debug, info};

use windows::s;

use crate::hooks::{handle_demand3, handle_do_residence_consumption_stuff};

pub mod api;
pub mod ffi;
pub mod hooks;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
    debug!("start()");
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0x16924da, handle_demand3 as usize).unwrap();
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0x99ac2d, handle_do_residence_consumption_stuff as usize).unwrap();
    info!("Anno1800 agent startup completed sucessfully.");
    1
}

#[no_mangle]
pub unsafe extern "C" fn stop() -> u32 {
    debug!("stop()");
    ffi::unhook_call_rel32(s!("Anno1800.exe"), 0x16924da, 0xff122f01).unwrap();
    ffi::unhook_call_rel32(s!("Anno1800.exe"), 0x99ac2d, 0xfffdfd8e).unwrap();
    info!("Stop completed sucessfully");
    1
}
