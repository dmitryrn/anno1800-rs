use crate::api::{get_module_offset, offsets::STATIC_TEXT_MANAGER_INNER_PTR_OFFSET, struct188::Struct118, AnnoPtr};

pub struct TextManagerInnerPtr {
    pub address: u64,
}

impl TextManagerInnerPtr {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub unsafe fn from_static_ptr() -> Self {
        Self::new(*(get_module_offset(STATIC_TEXT_MANAGER_INNER_PTR_OFFSET) as *const u64))
    }

    pub fn get_struct_118(&self) -> Struct118 {
        Struct118 { address: self.address + 0x28 }
    }
}

impl AnnoPtr for TextManagerInnerPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
