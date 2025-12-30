use std::mem::transmute;

use crate::api::{
    get_module_offset,
    offsets::{STATIC_TRADE_CONTRACT_MANAGER_INNER_PTR_OFFST, TRADE_CONTRACTS_GET_TRADE_CONTRACTS_OF_ISLAND_OFFSETR},
    trade_contract::TradeContractPtr,
    AnnoPtr,
};

#[repr(C)]
pub struct TradeContractManagerPtr {
    pub address: u64,
}

impl TradeContractManagerPtr {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub unsafe fn from_static_ptr() -> Self {
        Self::new(*(get_module_offset(STATIC_TRADE_CONTRACT_MANAGER_INNER_PTR_OFFST) as *const u64))
    }

    pub unsafe fn get_contracts_of(&self, island_id: u16) -> u64 {
        let get_trade_contracts_of_island: extern "C" fn(trade_contract_manager_ptr: u64, island_id: u16) -> u64 =
            unsafe { transmute(get_module_offset(TRADE_CONTRACTS_GET_TRADE_CONTRACTS_OF_ISLAND_OFFSETR) as usize) };
        get_trade_contracts_of_island(self.address, island_id)
    }

    pub unsafe fn get_contracts_of_island(island_id: u16) -> TradeContractPtr {
        TradeContractPtr {
            address: TradeContractManagerPtr::from_static_ptr().get_contracts_of(island_id),
        }
    }
}

impl AnnoPtr for TradeContractManagerPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
