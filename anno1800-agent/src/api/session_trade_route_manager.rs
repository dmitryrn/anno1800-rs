use crate::api::{hash_map::HashMapPtr, AnnoPtr};

pub struct SessionTradeRouteManagerPtr {
    pub address: u64,
}

impl SessionTradeRouteManagerPtr {
    pub unsafe fn get_hashmap(&self) -> HashMapPtr {
        HashMapPtr::new(self.get(0x88))
    }
}

impl AnnoPtr for SessionTradeRouteManagerPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
