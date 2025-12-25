use crate::api::text_manager::TextManagerInnerPtr;

use super::{string_buffer::StringBufferPtr, AnnoPtr};

pub struct IslandPtr {
    pub address: u64,
}

impl IslandPtr {
    pub unsafe fn get_owner_index(&self) -> u16 {
        self.get(0x004e)
    }

    pub unsafe fn get_custom_name(&self) -> StringBufferPtr {
        StringBufferPtr::new(self.address + 0x0120)
    }

    pub unsafe fn get_city_name_guid(&self) -> u32 {
        self.get(0x0140)
    }

    pub unsafe fn get_name(&self) -> String {
        let custom_name = self.get_custom_name();
        if custom_name.get_len() > 0 {
            custom_name.get_string()
        } else {
            TextManagerInnerPtr::from_static_ptr().get_struct_118().get_text(self.get_city_name_guid(), 0)
        }
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
