use super::{ware_type::WareType, AnnoPtr};
use std::fmt::Debug;

pub struct WareProductionExtraPtr {
    pub address: u64,
}

impl WareProductionExtraPtr {
    pub fn get_building_type(&self) -> WareType {
        self.get(0x0000)
    }

    pub fn get_value(&self) -> f32 {
        self.get(0x0004)
    }
}

impl Debug for WareProductionExtraPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProductionBuildingBuffPtr")
            .field("address", &format!("{:#018x}", &self.address))
            .field("building_type", &self.get_building_type())
            .field("value", &format!("{:.2}", &self.get_value()))
            .finish()
    }
}

impl AnnoPtr for WareProductionExtraPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
