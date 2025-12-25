use std::mem::transmute;

use crate::api::{get_module_offset, offsets::STRUCT188_GET_TEXT_OFFSET, string_buffer::StringBufferPtr, AnnoPtr};

pub struct Struct118 {
    pub address: u64,
}

impl Struct118 {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub unsafe fn get_text(&self, key: u32, unknown: u8) -> String {
        let get_text: extern "C" fn(struct188_ptr: u64, key: u32, unknown: u8) -> u64 =
            unsafe { transmute(get_module_offset(STRUCT188_GET_TEXT_OFFSET) as usize) };
        let string_buffer_ptr = get_text(self.address, key, unknown);
        let string_buffer = StringBufferPtr::new(string_buffer_ptr);
        string_buffer.get_string()
    }
}

impl AnnoPtr for Struct118 {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
