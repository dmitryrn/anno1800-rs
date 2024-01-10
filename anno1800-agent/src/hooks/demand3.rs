use std::slice;

use crate::api::{
    area_object_manager::AreaObjectManagerPtr,
    ware_type::{BLUEPRINT, CULTIVATION_AREA, DEPOSIT},
};

use super::{send, AnnoMessage, ExtraProductionMessage, ProductionMessage};

pub unsafe fn handle_demand3(area_object_manager: AreaObjectManagerPtr) {
    let class59 = area_object_manager.get_class59();
    let string_buffer = class59.get_string_buffer();
    let buf = string_buffer.get_buf();
    let island_name = String::from_utf16_lossy(slice::from_raw_parts(buf as *const u16, string_buffer.get_len() as _));
    let class46 = area_object_manager.get_class46();
    let class20 = class46.get_class20(0x0301);
    let productions = class20.get_productions();

    for production in productions {
        let ware_type = production.get_ware_type();
        if ware_type == BLUEPRINT {
            continue;
        }
        let potential_production = production.get_potential_production();
        let inputs = production.get_inputs();
        let buffs = production.get_buffs();
        let message = AnnoMessage {
            production: Some(ProductionMessage {
                address: production.address,
                island: island_name.clone(),
                ware_type: ware_type.into(),
                ware_string: format!("{:?}", ware_type),
                potential_production,
                potential_extra_production: buffs
                    .iter()
                    .map(|e| ExtraProductionMessage {
                        ware_type: e.get_ware_type().into(),
                        ware_string: format!("{:?}", e.get_ware_type()),
                        potential_production: e.get_value(),
                    })
                    .collect(),
                inputs: inputs
                    .iter()
                    .filter(|e| e.get_ware_type() != DEPOSIT && e.get_ware_type() != CULTIVATION_AREA)
                    .map(|e| e.get_ware_type().into())
                    .collect(),
            }),
            residence_consumption: None,
        };
        send(&format!("{}\n", &serde_json::to_string(&message).unwrap()));
    }
}
