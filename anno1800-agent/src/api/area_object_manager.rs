use super::{class46::Class46Ptr, island::IslandPtr, AnnoPtr};

pub struct AreaObjectManagerPtr {
    pub address: u64,
}

impl AreaObjectManagerPtr {
    pub unsafe fn get_island(&self) -> IslandPtr {
        let address: u64 = self.get(0x0038);
        IslandPtr::new(address)
    }

    pub unsafe fn get_class46(&self) -> Class46Ptr {
        let address: u64 = self.get(0x0548);
        Class46Ptr::new(address)
    }
}

impl AnnoPtr for AreaObjectManagerPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
