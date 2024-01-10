use super::{ware_type::WareType, AnnoPtr};
use std::fmt::Debug;

pub struct Class30Ptr {
    pub address: u64,
}

impl Class30Ptr {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub unsafe fn get_ware_type(&self) -> WareType {
        self.get(0x00)
    }

    pub unsafe fn get_demand_per_second(&self) -> f32 {
        self.get(0x0c)
    }
}

impl AnnoPtr for Class30Ptr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

impl Debug for Class30Ptr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.debug_struct("Class30")
                .field("address", &format!("{:#018x}", &self.address))
                .field("ware_type", &format!("{:?}", &self.get_ware_type()))
                .field("demand_per_second", &format!("{:.2}", &self.get_demand_per_second()))
                .finish()
        }
    }
}
