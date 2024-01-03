use log::debug;
use std::{collections::HashMap, mem::transmute, net::UdpSocket, sync::OnceLock};
use windows::{s, Win32::System::LibraryLoader::GetModuleHandleA};

use crate::api::{class4::Class4Ptr, class46::Class46Ptr, get_module_base, AnnoPtr, BuildingType};

static CELL: OnceLock<UdpSocket> = OnceLock::new();

pub unsafe extern "fastcall" fn handle_update_potential_production_hook(class4_ptr: u64) {
    let class4 = Class4Ptr::new(class4_ptr);
    let socket = get_socket();
    socket.send_to(format!("{:?}\n", class4).as_bytes(), "192.168.178.33:1800").unwrap();
    let call_base = GetModuleHandleA(s!("Anno1800.exe")).unwrap();
    let call_address = call_base.0 as usize + 0xd4e400;
    let orig: extern "fastcall" fn(class4_ptr: u64) = unsafe { transmute(call_address) };
    orig(class4_ptr);
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
        let class46 = Class46Ptr::new(class46_ptr);
        let class20 = class46.get_class20(weird_id);
        // send(&format!("handle_demand2_loop {:?}\n", class20));
        let class4s = class20.get_class4s();
        let mut buf = format!("handle_demand2_loop class20={:#018x} buildings={}\n", class20.address, class4s.len());
        let mut map: HashMap<BuildingType, (usize, f32)> = HashMap::new();
        for class4 in class4s {
            let building_type = class4.get_building_type();
            let potential_production = class4.get_prod_thingy().get_class34(&building_type).get_potential_production();
            map.entry(building_type).or_default().0 += 1;
            map.entry(building_type).or_default().1 += potential_production;
            // buf.push_str(&format!("    {:#018x} {:?} ({:.02}/min) \n", class4.address, building_type, potential_production))
        }
        for (key, val) in map.iter() {
            buf.push_str(&format!("    {:?}: {}\t{}/min\n", key, val.0, val.1))
        }
        if class20.address == 0x000002e0a803ade0 {
            send(&buf);
        }
    }
    let call_base = get_module_base();
    let call_address = call_base + 0xcabc70;
    let orig: extern "fastcall" fn(class46_ptr: u64, weird_id: u32, a3: u64, a4: u64) = unsafe { transmute(call_address as usize) };
    orig(class46_ptr, weird_id, a3, a4);
}

fn get_socket() -> &'static UdpSocket {
    CELL.get_or_init(|| {
        debug!("creating udp socket");
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        debug!("creating udp socket done");
        socket
    })
}

fn send(str: &str) {
    let socket = get_socket();
    socket.send_to(str.as_bytes(), "192.168.178.33:1800").unwrap();
}
