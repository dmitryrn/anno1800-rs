use std::fmt::Debug;

pub struct Class4 {
    pub address: u64,
}

impl Class4 {
    pub fn new(address: u64) -> Self {
        Self { address }
    }
}

impl Debug for Class4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class4").field("address", &self.address).finish()
    }
}
