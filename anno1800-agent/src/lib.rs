#![allow(clippy::missing_safety_doc)]
use std::{ffi::CStr, os::raw::c_char, panic};

use log::{debug, error, info};

use windows::s;

use crate::{
    api::offsets::TRACE_CONTRACT_MANAGER_DO_TRADE_CONTRACT_STUFF_OFFSET,
    hooks::{handle_demand3, handle_do_residence_consumption_stuff, handle_trade_contract, handle_trade_route_vehicle_on_game_tick_get_trade_route},
};

pub mod api;
pub mod ffi;
pub mod hooks;

#[no_mangle]
pub unsafe extern "C" fn set_host(host: *const c_char) -> u32 {
    if host.is_null() {
        return 0;
    }

    match CStr::from_ptr(host).to_str() {
        Ok(host) => {
            hooks::set_host(host);
            1
        }
        Err(e) => {
            error!("invalid host: {e}");
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
    panic::set_hook(Box::new(|p| {
        error!("panic: {p}");
    }));
    debug!("start()");
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0x169751a, handle_demand3 as usize).unwrap();
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0x97F790, handle_do_residence_consumption_stuff as usize).unwrap();
    ffi::hook_call_rel32(s!("Anno1800.exe"), 0xDC3BD6, handle_trade_route_vehicle_on_game_tick_get_trade_route as usize).unwrap();
    ffi::hook_call_rel32(
        s!("Anno1800.exe"),
        TRACE_CONTRACT_MANAGER_DO_TRADE_CONTRACT_STUFF_OFFSET,
        handle_trade_contract as usize,
    )
    .unwrap();
    info!("Anno1800 agent startup completed sucessfully.");
    1
}

#[no_mangle]
pub unsafe extern "C" fn stop() -> u32 {
    debug!("stop()");
    //ffi::unhook_call_rel32(s!("Anno1800.exe"), 0x16924da, 0xff122f01).unwrap();
    //ffi::unhook_call_rel32(s!("Anno1800.exe"), 0x99ac2d, 0xfffdfd8e).unwrap();
    info!("Stop completed sucessfully");
    1
}
