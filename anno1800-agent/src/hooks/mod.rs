use serde::{Deserialize, Serialize};
use std::{mem::transmute, net::UdpSocket, sync::OnceLock};

use crate::api::{
    area_object_manager::AreaObjectManagerPtr, area_residence_consumption_manager::AreaResidenceConsumptionManagerPtr, get_module_offset, AnnoPtr,
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
    island_owner: u16,
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
    island_owner: u16,
    potential_consumption: f32,
    potential_extra_production: Vec<ExtraProductionMessage>,
    inputs: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct ResidenceConsumptionsMessage {
    island: String,
    island_owner: u16,
    consumptions: Vec<ResidenceConsumptionMessage>,
}

#[derive(Serialize, Deserialize)]
struct ResidenceConsumptionMessage {
    ware_type: u32,
    ware_string: String,
    consumption: f32,
}

pub unsafe extern "C" fn handle_demand3(area_object_manager_ptr: u64, weird_id: u32, a3: u32, a4: u64) {
    demand3::handle_demand3(AreaObjectManagerPtr::new(area_object_manager_ptr));
    let call_address: u64 = get_module_offset(0x7BA460);
    let orig: extern "C" fn(area_object_manager_ptr: u64, weird_id: u32, a3: u32, a4: u64) = unsafe { transmute(call_address as usize) };
    orig(area_object_manager_ptr, weird_id, a3, a4);
}

pub unsafe extern "C" fn handle_do_residence_consumption_stuff(area_residence_consumption_manager_ptr: u64) {
    handle_residences(AreaResidenceConsumptionManagerPtr::new(area_residence_consumption_manager_ptr));
    let call_address = get_module_offset(0x97FDA0);
    let orig: extern "C" fn(area_object_manager_ptr: u64) = unsafe { transmute(call_address as usize) };
    orig(area_residence_consumption_manager_ptr);
}

fn get_socket() -> &'static UdpSocket {
    CELL.get_or_init(|| UdpSocket::bind("0.0.0.0:0").unwrap())
}

fn send(str: &str) {
    let socket = get_socket();
    socket.send_to(str.as_bytes(), "127.0.0.1:1800").unwrap();
}
