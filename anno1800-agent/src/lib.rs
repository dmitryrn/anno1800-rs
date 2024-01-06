#![allow(clippy::missing_safety_doc)]
use log::{debug, info};

use windows::s;

use crate::hooks::{handle_demand2_loop, handle_demand3};

pub mod api;
pub mod ffi;
pub mod hooks;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
    debug!("start()");
    // For good measure
    // ffi::hook_call_rel32(s!("Anno1800.exe"), 0xd44c79, handle_update_potential_production_hook as usize).unwrap();
    // This does not cover all objects going into `loop_over_prod_buildings`, but some
    //ffi::hook_call_rel32(s!("Anno1800.exe"), 0x7B0F3C, handle_loop_over_islands_hook as usize).unwrap();
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0x7B54F3, handle_demand2_loop as usize).unwrap();
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0x789286, handle_demand3 as usize).unwrap();
    info!("Anno1800 agent startup completed sucessfully.");
    1
}

#[no_mangle]
pub unsafe extern "C" fn stop() -> u32 {
    debug!("stop()");
    //ffi::unhook_call_rel32(s!("Anno1800.exe"), 0xd44c79, 0x00009782).unwrap();
    //ffi::unhook_call_rel32(s!("Anno1800.exe"), 0x7b0f2b, 0x004f9d93).unwrap();
    ffi::unhook_call_rel32(s!("Anno1800.exe"), 0x7B54F3, 0x004F6778).unwrap();
    ffi::unhook_call_rel32(s!("Anno1800.exe"), 0x789286, 0x0002C155).unwrap();
    info!("Stop completed sucessfully");
    1
}
