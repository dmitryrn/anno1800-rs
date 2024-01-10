use std::slice;

use super::AnnoPtr;

pub struct StringBufferPtr {
    pub address: u64,
}

impl StringBufferPtr {
    pub unsafe fn get_buf(&self) -> u64 {
        if self.get_len() >= 8 {
            self.get(0x0000)
        } else {
            self.address
        }
    }

    pub unsafe fn get_len(&self) -> u64 {
        self.get(0x0010)
    }

    pub unsafe fn get_string(&self) -> String {
        String::from_utf16_lossy(slice::from_raw_parts(self.get_buf() as *const u16, self.get_len() as _))
    }
}

impl AnnoPtr for StringBufferPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
