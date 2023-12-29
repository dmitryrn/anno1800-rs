use std::fmt::Debug;

pub struct Class4 {
    pub address: u64,
}

impl Class4 {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub fn get_efficiency_factor(&self) -> f32 {
        unsafe { ((self.address + 0x01f0) as *const f32).read_volatile() }
    }

    pub fn get_productivity(&self) -> u32 {
        unsafe {((self.address + 0x01f4) as *const u32).read_volatile() }
    }

    pub fn get_millis_per_cycle(&self) -> u64 {
        unsafe {((self.address + 0x01f8) as *const u64).read_volatile() }
    }
}

impl Debug for Class4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class4")
            .field("address", &self.address)
            .field("efficiency_factor", &self.get_efficiency_factor())
            .field("productivity", &self.get_productivity())
            .field("millis_per_cycle", &self.get_millis_per_cycle())
            .finish()
    }
}
