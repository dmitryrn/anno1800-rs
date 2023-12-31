use std::fmt::Debug;

pub struct Class33 {
    pub address: u64,
}

impl Class33 {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}
