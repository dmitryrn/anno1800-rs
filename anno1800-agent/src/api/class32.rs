use std::fmt::Debug;

use super::ware_type::WareType;

pub struct Class32 {
    pub address: u64,
}

impl Class32 {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub fn get_ware_type(&self) -> WareType {
        self.get(0x0000)
    }

    pub fn get_4(&self) -> f32 {
        self.get(0x0004)
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

impl Debug for Class32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class32")
            .field("address", &format!("{:#018x}", &self.address))
            .field("ware_type", &format!("{:?}", &self.get_ware_type()))
            .field("4", &format!("{:.2}", &self.get_4()))
            .finish()
    }
}
