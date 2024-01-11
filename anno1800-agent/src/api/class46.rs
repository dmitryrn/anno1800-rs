use super::{
    consumption_buildings::ConsumptionBuildingsPtr, energy_buildings::EnergyBuildingsPtr, hash_map::HashMapPtr, production_buildings::ProductionBuildingsPtr,
    AnnoPtr,
};
use std::fmt::Debug;

const CONSUMPTION_BUILDINGS_INDEX: u32 = 0x02fd;
const PRODUCTION_BUILDINGS_INDEX: u32 = 0x0301;
const ENERGY_BUILDINGS_INDEX: u32 = 0x0315;

pub struct Class46Ptr {
    pub address: u64,
}

impl Class46Ptr {
    pub fn get_vtable(&self) -> u64 {
        self.get(0x0000)
    }

    pub fn get_hashmap_ptr(&self) -> HashMapPtr {
        unsafe { HashMapPtr::new(self.address + 0x0048) }
    }

    pub fn get_field_58(&self) -> u64 {
        self.get(0x0058)
    }

    pub fn get_field_70(&self) -> u64 {
        self.get(0x0070)
    }

    pub unsafe fn get_production_buildings(&self) -> ProductionBuildingsPtr {
        let map = self.get_hashmap_ptr();
        let address = map.get_entry(PRODUCTION_BUILDINGS_INDEX);
        ProductionBuildingsPtr::new(address)
    }

    pub unsafe fn get_consumption_buildings(&self) -> ConsumptionBuildingsPtr {
        let map = self.get_hashmap_ptr();
        let address = map.get_entry(CONSUMPTION_BUILDINGS_INDEX);
        ConsumptionBuildingsPtr::new(address)
    }

    pub unsafe fn get_energy_buildings(&self) -> EnergyBuildingsPtr {
        let map = self.get_hashmap_ptr();
        let address = map.get_entry(ENERGY_BUILDINGS_INDEX);
        EnergyBuildingsPtr::new(address)
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

impl AnnoPtr for Class46Ptr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

impl Debug for Class46Ptr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class46Ptr").field("address", &format!("{:#018x}", &self.address)).finish()
    }
}
