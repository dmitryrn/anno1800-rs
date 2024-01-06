use std::{mem::transmute, net::UdpSocket, sync::OnceLock};
use windows::{s, Win32::System::LibraryLoader::GetModuleHandleA};

use crate::api::{area_object_manager::AreaObjectManagerPtr, get_module_base, get_module_offset, ware_production::WareProductionPtr, AnnoPtr};
pub mod demand2_loop;
pub mod demand3;

static CELL: OnceLock<UdpSocket> = OnceLock::new();

pub unsafe extern "fastcall" fn handle_update_potential_production_hook(production_building_ptr: u64) {
    let production_building = WareProductionPtr::new(production_building_ptr);
    let socket = get_socket();
    socket
        .send_to(format!("{:?}\n", production_building).as_bytes(), "192.168.178.33:1800")
        .unwrap();
    let call_base = GetModuleHandleA(s!("Anno1800.exe")).unwrap();
    let call_address = call_base.0 as usize + 0xd4e400;
    let orig: extern "fastcall" fn(class4_ptr: u64) = unsafe { transmute(call_address) };
    orig(production_building_ptr);
}

/*
pub unsafe extern "fastcall" fn handle_loop_over_islands_hook(class11_ptr: u64, a2: u32) {
    let class11 = Class11::new(class11_ptr);
    let socket = get_socket();
    socket.send_to(format!("{:?}\n", class11).as_bytes(), "192.168.178.33:1800").unwrap();
    let call_base = GetModuleHandleA(s!("Anno1800.exe")).unwrap();
    let call_address = call_base.0 as usize + 0xcaac00;
    let orig: extern "fastcall" fn(class11_ptr: u64, a2: u32) = unsafe { transmute(call_address) };
    orig(class11_ptr, a2);
}
*/

pub unsafe extern "fastcall" fn handle_demand2_loop(class46_ptr: u64, weird_id: u32, a3: u64, a4: u64) {
    if weird_id == 0x301 {
        demand2_loop::handle_demand2_loop(class46_ptr, weird_id)
    }
    let call_base = get_module_base();
    let call_address = call_base + 0xcabc70;
    let orig: extern "fastcall" fn(class46_ptr: u64, weird_id: u32, a3: u64, a4: u64) = unsafe { transmute(call_address as usize) };
    orig(class46_ptr, weird_id, a3, a4);
}

pub unsafe extern "fastcall" fn handle_demand3(area_object_manager_ptr: u64, weird_id: u32, a3: u32, a4: u64) {
    demand3::handle_demand3(AreaObjectManagerPtr::new(area_object_manager_ptr));
    let call_address = get_module_offset(0x7B53E0);
    let orig: extern "fastcall" fn(area_object_manager_ptr: u64, weird_id: u32, a3: u32, a4: u64) = unsafe { transmute(call_address as usize) };
    orig(area_object_manager_ptr, weird_id, a3, a4);
}

fn get_socket() -> &'static UdpSocket {
    CELL.get_or_init(|| UdpSocket::bind("0.0.0.0:0").unwrap())
}

fn send(str: &str) {
    let socket = get_socket();
    socket.send_to(str.as_bytes(), "192.168.178.33:1800").unwrap();
}
