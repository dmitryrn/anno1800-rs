use std::fmt::Debug;

use log::warn;

use super::{class4::Class4Ptr, get_module_base, BoxedArrayListPtr};

pub enum Class20 {
    Class20_1Ptr(Class20_1Ptr),
    Unknown(u64),
}
pub struct Class20_1Ptr {
    pub address: u64,
}

impl Class20 {
    pub unsafe fn new(address: u64) -> Self {
        let vtable_address = (address as *const u64).read_volatile();
        let vtable_offset = vtable_address - get_module_base();
        match vtable_offset {
            Class20_1Ptr::VTABLE_OFFSET => Class20::Class20_1Ptr(Class20_1Ptr::new(address)),
            _ => Class20::Unknown(address),
        }
    }
}

impl Class20_1Ptr {
    const VTABLE_OFFSET: u64 = 0x510E550;

    pub unsafe fn new(address: u64) -> Self {
        let obj = Self { address };
        let vtable = obj.get_vtable();
        if vtable - get_module_base() != Self::VTABLE_OFFSET {
            warn!("Unexpected Class20_1Ptr vtable {vtable:#018x}")
        }
        obj
    }

    pub fn get_vtable(&self) -> u64 {
        self.get(0x0000)
    }

    pub fn get_class4s(&self) -> Vec<Class4Ptr> {
        let list = unsafe { BoxedArrayListPtr::new(self.address + 0x28) };
        list.get_all::<Class4Ptr>()
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

impl Debug for Class20_1Ptr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class20Ptr")
            .field("address", &format!("{:#018x}", &self.address))
            .field("class4s", &self.get_class4s())
            .finish()
    }
}
