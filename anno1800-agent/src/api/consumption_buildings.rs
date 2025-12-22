use std::fmt::Debug;

use log::warn;

use super::{array_list::ArrayListPtr, consumption_building::ConsumptionBuildingPtr, get_module_base, AnnoPtr};

pub struct ConsumptionBuildingsPtr {
    pub address: u64,
}

impl ConsumptionBuildingsPtr {
    const VTABLE_OFFSET: u64 = 0x511A748;

    pub unsafe fn new(address: u64) -> Self {
        let obj = Self { address };
        let vtable = obj.get_vtable();
        if vtable - get_module_base() != Self::VTABLE_OFFSET {
            warn!("Unexpected ConsumptionBuildingsPtr vtable {vtable:#018x}")
        }
        obj
    }

    pub fn get_vtable(&self) -> u64 {
        self.get(0x0000)
    }

    pub fn get_production_building_list(&self) -> Vec<*const ConsumptionBuildingPtr> {
        unsafe { ArrayListPtr::new(self.address + 0x28) }.get_all()
    }

    pub fn get_vec(&self) -> Vec<ConsumptionBuildingPtr> {
        unsafe { self.get_production_building_list().iter().map(|e| **e).collect() }
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

impl Debug for ConsumptionBuildingsPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConsumptionBuildingsPtr")
            .field("address", &format!("{:#018x}", &self.address))
            .field("buildings", &self.get_vec())
            .finish()
    }
}
