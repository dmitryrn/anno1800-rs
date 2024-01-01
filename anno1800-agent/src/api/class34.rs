use std::fmt::Debug;

pub struct Class34 {
    pub address: u64,
}

impl Class34 {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    /// Gets constantly updated, but *never* read anywhere else except in update_potential_production_or_consumption
    pub fn get_potential_production(&self) -> f32 {
        self.get(0x0000)
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

impl Debug for Class34 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class34")
            .field("address", &format!("{:#018x}", &self.address))
            .field("potential_production", &format!("{:.2}", &self.get_potential_production()))
            .finish()
    }
}
