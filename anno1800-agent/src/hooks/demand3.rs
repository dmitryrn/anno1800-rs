use std::slice;

use crate::api::{
    area_object_manager::AreaObjectManagerPtr,
    consumption_building::ConsumptionBuildingPtr,
    production_building::ProductionBuildingPtr,
    ware_type::{BLUEPRINT, CULTIVATION_AREA, DEPOSIT},
};

use super::{send, AnnoMessage, ConsumptionMessage, ExtraProductionMessage, ProductionMessage};

pub unsafe fn handle_demand3(area_object_manager: AreaObjectManagerPtr) {
    let class59 = area_object_manager.get_class59();
    let string_buffer = class59.get_string_buffer();
    let buf = string_buffer.get_buf();
    let island_name = String::from_utf16_lossy(slice::from_raw_parts(buf as *const u16, string_buffer.get_len() as _));
    if island_name != "T_Bennihausen" {
        return;
    }
    let class46 = area_object_manager.get_class46();
    handle_production_buildings(&island_name, &class46.get_production_buildings().get_vec(), true);
    handle_consumption_buildings(&island_name, &class46.get_consumption_buildings().get_vec());
}

pub unsafe fn handle_consumption_buildings(island_name: &str, productions: &[ConsumptionBuildingPtr]) {
    send(&format!("consumptions {}\n", productions.len()));
    for production in productions {
        let potential_consumption = production.get_cycles_per_minute();
        let inputs = production.get_inputs();
        let buffs = production.get_buffs();
        let message = AnnoMessage {
            production_building: None,
            consumption_building: Some(ConsumptionMessage {
                address: production.address,
                island: island_name.to_owned(),
                potential_consumption,
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

pub unsafe fn handle_production_buildings(island_name: &str, buildings: &[ProductionBuildingPtr], ignore_no_output_ware: bool) {
    send(&format!("productions {}\n", buildings.len()));
    for production in buildings {
        let ware_type = production.get_ware_type();
        if ignore_no_output_ware && ware_type == BLUEPRINT {
            continue;
        }
        let potential_production = production.get_potential_production();
        let inputs = production.get_inputs();
        let buffs = production.get_buffs();
        let message = AnnoMessage {
            production_building: Some(ProductionMessage {
                address: production.address,
                island: island_name.to_owned(),
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
            consumption_building: None,
            residence_consumption: None,
        };
        send(&format!("{}\n", &serde_json::to_string(&message).unwrap()));
    }
}
