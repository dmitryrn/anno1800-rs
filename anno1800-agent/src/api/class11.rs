use std::fmt::Debug;

use super::class20::{Class20, Class20_1Ptr};

pub struct Class11 {
    pub address: u64,
}

impl Class11 {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub fn get_class12_ptrs(&self) -> *const u64 {
        self.get(0x0080)
    }

    pub fn get_first_class20_index_ptr(&self) -> *const u64 {
        self.get(0x0098)
    }

    pub fn get_last_class20index_ptr(&self) -> *const u64 {
        self.get(0x00A0)
    }

    pub fn get_production_buildings(&self) -> Vec<Class20_1Ptr> {
        let mut data = vec![];
        let mut current = self.get_first_class20_index_ptr();
        let last = self.get_last_class20index_ptr();
        let ptrs = self.get_class12_ptrs();
        while current != last {
            unsafe {
                let index = current.read_volatile();
                let address = ptrs.offset(index as isize).read_volatile();
                match Class20::new(address) {
                    Class20::Class20_1Ptr(ptr) => data.push(ptr),
                    _ => {}
                }
                current = current.add(1);
            }
        }
        data
    }

    fn get<T>(&self, offset: u64) -> T {
        unsafe { ((self.address + offset) as *const T).read_volatile() }
    }
}

impl Debug for Class11 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class20ptrs = self.get_production_buildings();
        f.debug_struct("Class11")
            .field("address", &format!("{:#018x}", &self.address))
            .field("class20ptrs", &class20ptrs)
            .finish()
    }
}
