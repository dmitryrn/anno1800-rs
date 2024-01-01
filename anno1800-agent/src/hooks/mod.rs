use log::debug;
use std::{mem::transmute, net::UdpSocket, sync::OnceLock};
use windows::{s, Win32::System::LibraryLoader::GetModuleHandleA};

use crate::api::{class11::Class11, class4::Class4};

static CELL: OnceLock<UdpSocket> = OnceLock::new();

pub unsafe extern "fastcall" fn handle_update_potential_production_hook(class4_ptr: u64) {
    let class4 = Class4::new(class4_ptr);
    let socket = get_socket();
    socket.send_to(format!("{:?}\n", class4).as_bytes(), "192.168.178.33:1800").unwrap();
    let call_base = GetModuleHandleA(s!("Anno1800.exe")).unwrap();
    let call_address = call_base.0 as usize + 0xd4e400;
    let orig: extern "fastcall" fn(class4_ptr: u64) = unsafe { transmute(call_address) };
    orig(class4_ptr);
}

pub unsafe extern "fastcall" fn handle_loop_over_islands_hook(class11_ptr: u64, a2: u32) {
    let class11 = Class11::new(class11_ptr);
    let socket = get_socket();
    socket.send_to(format!("{:?}\n", class11).as_bytes(), "192.168.178.33:1800").unwrap();
    let call_base = GetModuleHandleA(s!("Anno1800.exe")).unwrap();
    let call_address = call_base.0 as usize + 0xcaac00;
    let orig: extern "fastcall" fn(class11_ptr: u64, a2: u32) = unsafe { transmute(call_address) };
    orig(class11_ptr, a2);
}

fn get_socket() -> &'static UdpSocket {
    CELL.get_or_init(|| {
        debug!("creating udp socket");
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        debug!("creating udp socket done");
        socket
    })
}
