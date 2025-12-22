use super::{string_buffer::StringBufferPtr, AnnoPtr};

pub struct IslandPtr {
    pub address: u64,
}

impl IslandPtr {
    pub unsafe fn get_owner_index(&self) {
        self.get(0x004e)
    }

    pub unsafe fn get_custom_name(&self) -> StringBufferPtr {
        StringBufferPtr::new(self.address + 0x0120)
    }

    pub unsafe fn get_city_name_guid(&self) -> u32 {
        self.get(0x0140)
    }
}

impl AnnoPtr for IslandPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
