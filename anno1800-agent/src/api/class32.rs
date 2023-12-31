use std::fmt::Debug;

pub struct Class32 {
    pub address: u64,
}

impl Class32 {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub fn get_ware_id(&self) -> u32 {
        self.get(0x0000)
    }

    pub fn get_4(&self) -> u32 {
        self.get(0x016c)
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

impl Debug for Class32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class32")
            .field("address", &format!("{:#018x}", &self.address))
            .field("ware_id", &format!("{:#010x}", &self.get_ware_id()))
            .field("4", &format!("{:#10x}", &self.get_4()))
            .finish()
    }
}
