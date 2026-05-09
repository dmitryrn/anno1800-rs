use serde::{Deserialize, Serialize};
use std::{mem::transmute, net::UdpSocket, sync::OnceLock};

use crate::{
    api::{
        area_object_manager::AreaObjectManagerPtr, area_residence_consumption_manager::AreaResidenceConsumptionManagerPtr, get_module_offset,
        trade_contracts::TradeContractsPtr, trade_route::TradeRoutePtr, AnnoPtr,
    },
    hooks::{trade_contracts::handle_contracts, trade_routes::handle_trade_route},
};

use self::residence_consumption::handle_residences;
pub mod demand3;
pub mod residence_consumption;
pub mod trade_contracts;
pub mod trade_routes;

static CELL: OnceLock<UdpSocket> = OnceLock::new();
static HOST: OnceLock<String> = OnceLock::new();

#[derive(Serialize, Deserialize)]
struct AnnoMessage {
    production_building: Option<ProductionMessage>,
    consumption_building: Option<ConsumptionMessage>,
    residence_consumption: Option<ResidenceConsumptionsMessage>,
    trade_route: Option<TradeRouteMessage>,
    trade_contracts: Option<IslandTradeContractsMessage>,
}

#[derive(Serialize, Deserialize)]
struct IslandTradeContractsMessage {
    island_id: u16,
    contracts: Vec<IslandTradeContractMessage>,
}

#[derive(Serialize, Deserialize)]
struct IslandTradeContractMessage {
    export_product_type: u32,
    export_product_string: String,
    export_amount: u32,
    import_product_type: u32,
    import_product_string: String,
    import_amount: u32,
}

#[derive(Serialize, Deserialize)]
struct ProductionMessage {
    address: u64,
    island: String,
    island_id: u16,
    island_owner: u16,
    product_type: u32,
    product_string: String,
    potential_production: f32,
    potential_extra_production: Vec<ExtraProductionMessage>,
    inputs: Vec<InputMessage>,
}

#[derive(Serialize, Deserialize)]
struct ExtraProductionMessage {
    product_type: u32,
    product_string: String,
    potential_production: f32,
}

#[derive(Serialize, Deserialize)]
struct InputMessage {
    product_type: u32,
    product_string: String,
    multiplier: u32,
}

#[derive(Serialize, Deserialize)]
struct ConsumptionMessage {
    address: u64,
    island: String,
    island_id: u16,
    island_owner: u16,
    potential_consumption: f32,
    potential_extra_production: Vec<ExtraProductionMessage>,
    inputs: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct ResidenceConsumptionsMessage {
    island: String,
    island_id: u16,
    island_owner: u16,
    consumptions: Vec<ResidenceConsumptionMessage>,
}

#[derive(Serialize, Deserialize)]
struct ResidenceConsumptionMessage {
    product_type: u32,
    product_string: String,
    consumption: f32,
}

#[derive(Serialize, Deserialize)]
struct TradeRouteMessage {
    address: u64,
    name: String,
    owner_id: u16,
    stops: Vec<TradeRouteStopMessage>,
}

#[derive(Serialize, Deserialize)]
struct TradeRouteStopMessage {
    island_id: u16,
    slots: Vec<TradeRouteStopSlotMessage>,
}

#[derive(Serialize, Deserialize)]
struct TradeRouteStopSlotMessage {
    product_type: u32,
    product_string: String,
    amount: u32,
    action: u8,
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

pub unsafe extern "C" fn handle_trade_route_vehicle_on_game_tick_get_trade_route(a1: u64, a2: u32) -> u64 {
    let call_address = get_module_offset(0x8522E0);
    let orig: extern "C" fn(a1: u64, a2: u32) -> u64 = unsafe { transmute(call_address as usize) };
    let trade_route_address = orig(a1, a2);
    let trade_route = TradeRoutePtr::new(trade_route_address);
    handle_trade_route(trade_route);
    trade_route_address
}

pub unsafe extern "C" fn handle_trade_contract(trade_contract_manager: u64, island_id: u16) -> u64 {
    let call_address = get_module_offset(0x84CB70);
    let orig: extern "C" fn(trade_contract_manager: u64, island_id: u16) -> u64 = unsafe { transmute(call_address as usize) };
    let trade_contracts_address = orig(trade_contract_manager, island_id);
    if trade_contracts_address != 0 {
        let trade_contracts = TradeContractsPtr::new(trade_contracts_address);
        handle_contracts(trade_contracts);
    }
    trade_contracts_address
}

fn get_socket() -> &'static UdpSocket {
    CELL.get_or_init(|| UdpSocket::bind("0.0.0.0:0").unwrap())
}

pub fn set_host(host: &str) {
    let host = host.trim();
    let host = if host.is_empty() { "127.0.0.1" } else { host };
    let _ = HOST.set(host.to_string());
}

fn send(str: &str) {
    let socket = get_socket();
    let host = HOST.get().map(String::as_str).unwrap_or("127.0.0.1");
    socket.send_to(str.as_bytes(), format!("{host}:1800")).unwrap();
}
