use crate::api::{array_list::ArrayListPtr, trade_contract::TradeContractPtr, AnnoPtr};

pub struct TradeContractsPtr {
    pub address: u64,
}

impl TradeContractsPtr {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub unsafe fn get_island_id(&self) -> u16 {
        self.get(0x68)
    }

    pub fn get_contracts_list(&self) -> ArrayListPtr<TradeContractPtr> {
        unsafe { ArrayListPtr::new(self.address + 0x80) }
    }
}

impl AnnoPtr for TradeContractsPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
