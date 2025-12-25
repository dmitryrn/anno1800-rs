use std::fmt::Debug;

use super::{array_list::ArrayListPtr, class32::Class32, ware_production_extra::WareProductionExtraPtr, ware_type::WareType, AnnoPtr};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ProductionBuildingPtr {
    pub address: u64,
}

impl ProductionBuildingPtr {
    pub fn get_vtable(&self) -> u64 {
        self.get(0x0000)
    }

    pub fn get_field_8(&self) -> u64 {
        self.get(0x0008)
    }

    pub fn get_first_input(&self) -> u64 {
        self.get(0x0148)
    }

    pub fn get_last_input(&self) -> u64 {
        self.get(0x0150)
    }

    pub fn get_16c(&self) -> u32 {
        self.get(0x016c) // always 1?
    }

    pub fn get_ware_type(&self) -> WareType {
        self.get(0x0168)
    }

    pub fn get_buffs_list(&self) -> ArrayListPtr<WareProductionExtraPtr> {
        unsafe { ArrayListPtr::new(self.address + 0x0188) }
    }

    pub fn get_potential_productivity_factor(&self) -> f32 {
        self.get(0x01f0)
    }

    pub fn get_current_productivity_factor(&self) -> f32 {
        self.get(0x01f4)
    }

    pub fn get_millis_per_cycle(&self) -> u64 {
        self.get(0x01f8)
    }

    pub fn get_200(&self) -> u8 {
        self.get(0x0200)
    }

    pub fn get_inputs(&self) -> Vec<Class32> {
        let mut inputs = vec![];
        let mut input = self.get_first_input();
        let last_input = self.get_last_input();
        while input != last_input {
            inputs.push(unsafe { Class32::new(input) });
            input += 8;
        }
        inputs
    }

    pub fn get_buffs(&self) -> Vec<WareProductionExtraPtr> {
        self.get_buffs_list().get_all()
    }

    pub fn get_potential_production(&self) -> f32 {
        if self.get_200() == 0 {
            60000.0 / self.get_millis_per_cycle() as f32 * self.get_16c() as f32 * self.get_potential_productivity_factor()
        } else {
            0.0
        }
    }
}

impl Debug for ProductionBuildingPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProductionBuildingPtr")
            .field("address", &format!("{:#018x}", &self.address))
            .field("current_productivity_factor", &format!("{:.2}", &self.get_current_productivity_factor()))
            .field("potential_productivity_factor", &format!("{:.2}", &self.get_potential_productivity_factor()))
            .field("millis_per_cycle", &format!("{:05}", &self.get_millis_per_cycle()))
            .field("type", &self.get_ware_type())
            .field("inputs", &self.get_inputs())
            .finish()
    }
}

impl AnnoPtr for ProductionBuildingPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
