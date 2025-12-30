use crate::api::{ware_type::WareType, AnnoPtr};

#[repr(C)]
pub struct TradeContractPtr {
    pub address: u64,
}

impl TradeContractPtr {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub unsafe fn get_export_product_type(&self) -> WareType {
        self.get(0x08)
    }

    pub unsafe fn get_export_amount(&self) -> u32 {
        self.get(0x0c)
    }

    pub unsafe fn get_import_product_type(&self) -> WareType {
        self.get(0x14)
    }

    pub unsafe fn get_import_amount(&self) -> u32 {
        self.get(0x18)
    }
}

impl AnnoPtr for TradeContractPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
