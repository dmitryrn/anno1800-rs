use super::{string_buffer::StringBufferPtr, AnnoPtr};

pub struct Class59Ptr {
    pub address: u64,
}

impl Class59Ptr {
    pub unsafe fn get_string_buffer(&self) -> StringBufferPtr {
        StringBufferPtr::new(self.address + 0x0120)
    }
}

impl AnnoPtr for Class59Ptr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
