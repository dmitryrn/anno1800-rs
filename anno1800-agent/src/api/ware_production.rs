use std::fmt::Debug;

use super::{
    array_list::ArrayListPtr, class32::Class32, class33::Class33, class34::Class34, ware_production_extra::WareProductionExtraPtr, ware_type::WareType, AnnoPtr,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WareProductionPtr {
    pub address: u64,
}

impl WareProductionPtr {
    pub fn get_vtable(&self) -> u64 {
        self.get(0x0000)
    }

    pub fn get_field_8(&self) -> u64 {
        self.get(0x0008)
    }

    pub fn get_prod_thingy(&self) -> Class33 {
        unsafe { Class33::new(self.address + 0x00b0) }
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

    pub fn get_prod_class34(&self) -> Class34 {
        let class33 = self.get_prod_thingy();
        let building_type = self.get_ware_type();
        class33.get_class34(&building_type)
    }
}

impl Debug for WareProductionPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProductionBuildingPtr")
            .field("address", &format!("{:#018x}", &self.address))
            //.field("vtable", &format!("{:#018x}", &self.get_vtable()))
            //.field("field_8", &format!("{:#018x}", &self.get_field_8()))
            //.field("field_16c", &format!("{:#08x}", &self.get_16c()))
            .field("current_productivity_factor", &format!("{:.2}", &self.get_current_productivity_factor()))
            .field("potential_productivity_factor", &format!("{:.2}", &self.get_potential_productivity_factor()))
            .field("millis_per_cycle", &format!("{:05}", &self.get_millis_per_cycle()))
            .field("prod_class34", &self.get_prod_class34())
            .field("type", &self.get_ware_type())
            .field("inputs", &self.get_inputs())
            //.field("buffs", &self.get_buffs())
            .finish()
    }
}

impl AnnoPtr for WareProductionPtr {
    unsafe fn new(address: u64) -> Self {
        Self { address }
    }

    fn get_address(&self) -> u64 {
        self.address
    }
}
