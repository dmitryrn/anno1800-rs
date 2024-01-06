use super::{class59::Class59Ptr, AnnoPtr, string_buffer::StringBufferPtr};

pub struct AreaObjectManagerPtr {
    pub address: u64,
}

impl AreaObjectManagerPtr {
    pub unsafe fn get_class59(&self) -> Class59Ptr {
        let address: u64 = self.get(0x0038);
        Class59Ptr::new(address)
    }
}

impl AnnoPtr for AreaObjectManagerPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
