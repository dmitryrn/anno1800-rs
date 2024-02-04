use serde::{Deserialize, Serialize};
use std::{mem::transmute, net::UdpSocket, sync::OnceLock};
use windows::{s, Win32::System::LibraryLoader::GetModuleHandleA};

use crate::api::{
    area_object_manager::AreaObjectManagerPtr, area_residence_consumption_manager::AreaResidenceConsumptionManagerPtr, get_module_offset,
    production_building::ProductionBuildingPtr, AnnoPtr,
};

use self::residence_consumption::handle_residences;
pub mod demand3;
pub mod residence_consumption;

static CELL: OnceLock<UdpSocket> = OnceLock::new();

#[derive(Serialize, Deserialize)]
struct AnnoMessage {
    production_building: Option<ProductionMessage>,
    consumption_building: Option<ConsumptionMessage>,
    residence_consumption: Option<ResidenceConsumptionsMessage>,
}

#[derive(Serialize, Deserialize)]
struct ProductionMessage {
    address: u64,
    island: String,
    ware_type: u32,
    ware_string: String,
    potential_production: f32,
    potential_extra_production: Vec<ExtraProductionMessage>,
    inputs: Vec<InputMessage>,
}

#[derive(Serialize, Deserialize)]
struct ExtraProductionMessage {
    ware_type: u32,
    ware_string: String,
    potential_production: f32,
}

#[derive(Serialize, Deserialize)]
struct InputMessage {
    ware_type: u32,
    ware_string: String,
    multiplier: u32,
}

#[derive(Serialize, Deserialize)]
struct ConsumptionMessage {
    address: u64,
    island: String,
    potential_consumption: f32,
    potential_extra_production: Vec<ExtraProductionMessage>,
    inputs: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct ResidenceConsumptionsMessage {
    island: String,
    consumptions: Vec<ResidenceConsumptionMessage>,
}

#[derive(Serialize, Deserialize)]
struct ResidenceConsumptionMessage {
    ware_type: u32,
    ware_string: String,
    consumption: f32,
}

pub unsafe extern "fastcall" fn handle_update_potential_production_hook(production_building_ptr: u64) {
    let production_building = ProductionBuildingPtr::new(production_building_ptr);
    let socket = get_socket();
    socket
        .send_to(format!("{:?}\n", production_building).as_bytes(), "192.168.178.33:1800")
        .unwrap();
    let call_base = GetModuleHandleA(s!("Anno1800.exe")).unwrap();
    let call_address = call_base.0 as usize + 0xd4e400;
    let orig: extern "fastcall" fn(class4_ptr: u64) = unsafe { transmute(call_address) };
    orig(production_building_ptr);
}

pub unsafe extern "fastcall" fn handle_demand3(area_object_manager_ptr: u64, weird_id: u32, a3: u32, a4: u64) {
    demand3::handle_demand3(AreaObjectManagerPtr::new(area_object_manager_ptr));
    let call_address = get_module_offset(0x7b53e0);
    let orig: extern "fastcall" fn(area_object_manager_ptr: u64, weird_id: u32, a3: u32, a4: u64) = unsafe { transmute(call_address as usize) };
    orig(area_object_manager_ptr, weird_id, a3, a4);
}

pub unsafe extern "fastcall" fn handle_do_residence_consumption_stuff(area_residence_consumption_manager_ptr: u64, a2: u64, a3: u64, a4: u64) {
    handle_residences(AreaResidenceConsumptionManagerPtr::new(area_residence_consumption_manager_ptr));
    let call_address = get_module_offset(0x97a9c0);
    let orig: extern "fastcall" fn(area_object_manager_ptr: u64, weird_id: u64, a3: u64, a4: u64) = unsafe { transmute(call_address as usize) };
    orig(area_residence_consumption_manager_ptr, a2, a3, a4);
}

fn get_socket() -> &'static UdpSocket {
    CELL.get_or_init(|| UdpSocket::bind("0.0.0.0:0").unwrap())
}

fn send(str: &str) {
    let socket = get_socket();
    socket.send_to(str.as_bytes(), "127.0.0.1:1800").unwrap();
}
