use std::mem::transmute;

use windows::{s, Win32::System::LibraryLoader::GetModuleHandleA};

use super::{class34::Class34, WareType};

pub struct Class33 {
    pub address: u64,
}

impl Class33 {
    pub unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    pub fn get_class34(&self, building_type: &WareType) -> Class34 {
        unsafe {
            let call_base = GetModuleHandleA(s!("Anno1800.exe")).unwrap();
            let call_address = call_base.0 as usize + 0xd63fb0;
            let orig: extern "fastcall" fn(class33_ptr: u64, building_type_ptr: u64) -> u64 = transmute(call_address);
            Class34::new(orig(self.address, building_type as *const WareType as u64))
        }
    }
}
