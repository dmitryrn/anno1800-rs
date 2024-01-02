use super::{AnnoPtr, HashMapPtr};
use std::fmt::Debug;

pub struct Class46Ptr {
    pub address: u64,
}

impl Class46Ptr {
    pub fn get_vtable(&self) -> u64 {
        self.get(0x0000)
    }

    pub fn get_class20(&self, weird_id: u32) -> u64 {
        let map = self.get_hashmap_ptr();
        map.get_entry(weird_id)
    }

    pub fn get_hashmap_ptr(&self) -> HashMapPtr {
        unsafe { HashMapPtr::new(self.address + 0x0048) }
    }

    pub fn get_field_58(&self) -> u64 {
        self.get(0x0058)
    }

    pub fn get_field_70(&self) -> u64 {
        self.get(0x0070)
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

impl AnnoPtr for Class46Ptr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}

impl Debug for Class46Ptr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class46Ptr")
            .field("address", &format!("{:#018x}", &self.address))
            .field("class47", &format!("{:#018x}", &self.get_class20(0x301)))
            .finish()
    }
}
