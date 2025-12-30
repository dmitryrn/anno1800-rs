use std::sync::OnceLock;

use windows::{s, Win32::System::LibraryLoader::GetModuleHandleA};
pub mod area_object_manager;
pub mod area_residence_consumption_manager;
pub mod array_list;
pub mod class30;
pub mod class32;
pub mod class46;
pub mod consumption_building;
pub mod consumption_buildings;
pub mod energy_building;
pub mod energy_buildings;
pub mod hash_map;
pub mod island;
pub mod offsets;
pub mod production_building;
pub mod production_buildings;
pub mod session_trade_route_manager;
pub mod string_buffer;
pub mod struct188;
pub mod text_manager;
pub mod trade_contract;
pub mod trade_contract_manager;
pub mod trade_contracts;
pub mod trade_route;
pub mod ware_production_extra;
pub mod ware_type;

static ANNO1800_BASE: OnceLock<u64> = OnceLock::new();

pub fn get_module_base() -> u64 {
    *ANNO1800_BASE.get_or_init(|| {
        let base = unsafe { GetModuleHandleA(s!("Anno1800.exe")) }.unwrap();
        base.0.try_into().unwrap()
    })
}

pub fn get_module_offset(offset: u64) -> u64 {
    get_module_base() + offset
}

pub trait AnnoPtr {
    unsafe fn new(address: u64) -> Self;

    fn get_address(&self) -> u64;

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.get_address() + offset) as *const T).read_volatile() }
    }
}

impl<T> AnnoPtr for *const T {
    unsafe fn new(address: u64) -> Self {
        address as _
    }

    fn get_address(&self) -> u64 {
        *self as u64
    }
}
