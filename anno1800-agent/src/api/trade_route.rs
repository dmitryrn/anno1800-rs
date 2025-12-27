use crate::api::{AnnoPtr, array_list::ArrayListPtr, string_buffer::StringBufferPtr, ware_type::WareType};

pub struct TradeRoutePtr {
    pub address: u64,
}

pub struct TradeRouteStopPtr {
    pub address: u64,
}

pub struct TradeRouteStopSlotPtr {
    pub address: u64,
}

impl TradeRoutePtr {
    pub unsafe fn get_route_name(&self) -> String {
        StringBufferPtr::new(self.address + 0x18).get_string()
    }

    pub unsafe fn get_stops(&self) -> Vec<TradeRouteStopPtr> {
        let array_list = ArrayListPtr::<*const *const TradeRouteStopPtr>::new(self.address + 0x38);
        let ptrs = array_list.get_all();
        ptrs.iter().map(|e| TradeRouteStopPtr::new(**e as _)).collect()
    }

    pub unsafe fn get_owner_id(&self) -> u16 {
        self.get(0x82)
    }
}

impl TradeRouteStopPtr {
    pub unsafe fn get_island_id(&self) -> u16 {
        self.get(0x10)
    }

    pub unsafe fn get_slots(&self) -> Vec<TradeRouteStopSlotPtr> {
        let array_list = ArrayListPtr::<*const *const TradeRouteStopSlotPtr>::new(self.address + 0x18);
        let ptrs = array_list.get_all();
        ptrs.iter().map(|e| TradeRouteStopSlotPtr::new(**e as _)).collect()
    }
}

impl TradeRouteStopSlotPtr {
    pub unsafe fn get_product_type(&self) -> WareType {
        self.get(0x10)
    }

    pub unsafe fn get_action(&self) -> u8 {
        self.get(0x14)
    }

    pub unsafe fn get_amount(&self) -> u32 {
        self.get(0x18)
    }
}

impl AnnoPtr for TradeRoutePtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

impl AnnoPtr for TradeRouteStopPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

impl AnnoPtr for TradeRouteStopSlotPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
