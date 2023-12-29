use std::fmt::Debug;

pub struct Class4 {
    pub address: u64,
}

impl Class4 {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub fn get_16c(&self) -> u32 {
        self.get(0x016c) // always 1?
    }

    pub fn get_type(&self) -> u32 {
        self.get(0x0168)
    }

    pub fn get_potential_productivity_factor(&self) -> f32 {
        self.get(0x01f0)
    }

    pub fn get_current_productivity_factor(&self) -> f32 {
        self.get(0x01f4)
    }

    pub fn get_millis_per_cycle(&self) -> u64 {
        self.get(0x01f8)
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

impl Debug for Class4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class4")
            .field("address", &format!("{:#016x}", &self.address))
            .field("field_16c", &format!("{:#08x}", &self.get_16c()))
            .field("current_productivity_factor", &format!("{:.2}", &self.get_current_productivity_factor()))
            .field("potential_productivity_factor", &format!("{:.2}", &self.get_potential_productivity_factor()))
            .field("millis_per_cycle", &format!("{:05}", &self.get_millis_per_cycle()))
            .field("type", &format!("{:#08x}", &self.get_type()))
            .finish()
    }
}
