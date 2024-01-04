use super::{AnnoPtr, BuildingType};
use std::fmt::Debug;

pub struct ProductionBuildingBuffPtr {
    pub address: u64,
}

impl ProductionBuildingBuffPtr {
    pub fn get_building_type(&self) -> BuildingType {
        self.get(0x0000)
    }

    pub fn get_value(&self) -> f32 {
        self.get(0x0004)
    }
}

impl Debug for ProductionBuildingBuffPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProductionBuildingBuffPtr")
            .field("address", &format!("{:#018x}", &self.address))
            .field("building_type", &self.get_building_type())
            .field("value", &format!("{:.2}", &self.get_value()))
            .finish()
    }
}

impl AnnoPtr for ProductionBuildingBuffPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
