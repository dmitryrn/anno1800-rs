use super::{class30::Class30Ptr, island::IslandPtr, AnnoPtr};

pub struct AreaResidenceConsumptionManagerPtr {
    pub address: u64,
}

impl AreaResidenceConsumptionManagerPtr {
    pub unsafe fn get_island(&self) -> IslandPtr {
        let address: u64 = self.get(0x0038);
        IslandPtr::new(address)
    }

    pub unsafe fn get_class30_first(&self) -> u64 {
        self.get(0x40)
    }

    pub unsafe fn get_class30_last(&self) -> u64 {
        self.get(0x48)
    }

    pub unsafe fn get_class30s(&self) -> Vec<Class30Ptr> {
        let mut data = vec![];
        let last = self.get_class30_last();
        let mut current = self.get_class30_first();
        while current != last {
            let ptr = Class30Ptr::new(current);
            data.push(ptr);
            current += 0x38;
        }
        data
    }
}

impl AnnoPtr for AreaResidenceConsumptionManagerPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
