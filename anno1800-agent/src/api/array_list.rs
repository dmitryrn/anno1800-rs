use std::marker::PhantomData;

use super::AnnoPtr;

pub struct ArrayListPtr<T: AnnoPtr> {
    pub address: u64,
    phantom: PhantomData<T>,
}

impl<T: AnnoPtr> ArrayListPtr<T> {
    pub fn get_first_box(&self) -> u64 {
        self.get(0x00)
    }

    pub fn get_last_box(&self) -> u64 {
        self.get(0x08)
    }

    pub fn get_all_words(&self) -> Vec<T> {
        //TODO enforce that T is 64bit wide
        let mut elements = vec![];
        let mut current = self.get_first_box() as *const u64;
        let last = self.get_last_box() as *const u64;
        while current != last {
            unsafe {
                let ptr = T::new(current as u64);
                elements.push(ptr);
                current = current.add(1);
            }
        }
        elements
    }
}

impl<T: AnnoPtr> AnnoPtr for ArrayListPtr<T> {
    unsafe fn new(address: u64) -> Self {
        Self { address, phantom: PhantomData }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
