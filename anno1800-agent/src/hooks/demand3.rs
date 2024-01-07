use std::slice;

use serde::{Deserialize, Serialize};

use crate::api::area_object_manager::AreaObjectManagerPtr;

use super::send;

#[derive(Serialize, Deserialize)]
struct ProductionMessage {
    address: u64,
    island: String,
    ware_type: u32,
    potential_production: f32,
    potential_extra_production: Vec<ExtraProductionMessage>,
    inputs: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct ExtraProductionMessage {
    ware_type: u32,
    potential_production: f32,
}

pub unsafe fn handle_demand3(area_object_manager: AreaObjectManagerPtr) {
    let class59 = area_object_manager.get_class59();
    let string_buffer = class59.get_string_buffer();
    let buf = string_buffer.get_buf();
    let island_name = String::from_utf16_lossy(slice::from_raw_parts(buf as *const u16, string_buffer.get_len() as _));
    let class46 = area_object_manager.get_class46();
    let class20 = class46.get_class20(0x0301);
    let productions = class20.get_productions();

    for production in productions {
        let building_type = production.get_ware_type();
        let potential_production = production.get_potential_production();
        let inputs = production.get_inputs();
        let buffs = production.get_buffs();
        let message = ProductionMessage {
            address: production.address,
            island: island_name.clone(),
            ware_type: building_type.into(),
            potential_production,
            potential_extra_production: buffs
                .iter()
                .map(|e| ExtraProductionMessage {
                    ware_type: e.get_building_type().into(),
                    potential_production: e.get_value(),
                })
                .collect(),
            inputs: inputs.iter().map(|e| e.get_ware_type().into()).collect(),
        };
        send(&format!("{}\n", &serde_json::to_string(&message).unwrap()));
    }
}
