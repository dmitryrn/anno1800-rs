use super::AnnoPtr;

pub struct StringBufferPtr {
    pub address: u64,
}

impl StringBufferPtr {
    pub unsafe fn get_buf(&self) -> u64 {
        self.get(0x0000)
    }

    pub unsafe fn get_len(&self) -> u64 {
        self.get(0x0010)
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
