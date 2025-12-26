use crate::api::{
    area_object_manager::AreaObjectManagerPtr,
    consumption_building::ConsumptionBuildingPtr,
    energy_building::EnergyBuildingPtr,
    production_building::ProductionBuildingPtr,
    ware_type::{BLUEPRINT, CULTIVATION_AREA, DEPOSIT},
};

use super::{send, AnnoMessage, ConsumptionMessage, ExtraProductionMessage, InputMessage, ProductionMessage};

pub unsafe fn handle_demand3(area_object_manager: AreaObjectManagerPtr) {
    let island = area_object_manager.get_island();
    let island_name = island.get_name();
    let island_id = island.get_island_id();
    let island_owner = island.get_owner_index();
    let class46 = area_object_manager.get_class46();
    handle_production_buildings(&island_name, island_id, island_owner, &class46.get_production_buildings().get_vec());
    handle_consumption_buildings(&island_name, island_id, island_owner, &class46.get_consumption_buildings().get_vec());
    handle_energy_buildings(&island_name, island_id, island_owner, &class46.get_energy_buildings().get_vec());
}

pub unsafe fn handle_production_buildings(island_name: &str, island_id: u16, island_owner: u16, buildings: &[ProductionBuildingPtr]) {
    for production in buildings {
        let ware_type = production.get_ware_type();
        if ware_type == BLUEPRINT {
            continue;
        }
        let potential_production = production.get_potential_production();
        let inputs = production.get_inputs();
        let buffs = production.get_buffs();
        let message = AnnoMessage {
            production_building: Some(ProductionMessage {
                address: production.address,
                island: island_name.to_owned(),
                island_id,
                island_owner,
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
                    .map(|e| InputMessage {
                        ware_type: e.get_ware_type().into(),
                        ware_string: format!("{:?}", e.get_ware_type()),
                        multiplier: e.get_4(),
                    })
                    .collect(),
            }),
            consumption_building: None,
            residence_consumption: None,
        };
        send(&format!("{}\n", &serde_json::to_string(&message).unwrap()));
    }
}

pub unsafe fn handle_consumption_buildings(island_name: &str, island_id: u16, island_owner: u16, buildings: &[ConsumptionBuildingPtr]) {
    for production in buildings {
        let potential_consumption = production.get_cycles_per_minute();
        let inputs = production.get_inputs();
        let buffs = production.get_buffs();
        let message = AnnoMessage {
            production_building: None,
            consumption_building: Some(ConsumptionMessage {
                address: production.address,
                island: island_name.to_owned(),
                island_id,
                island_owner,
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

pub unsafe fn handle_energy_buildings(island_name: &str, island_id: u16, island_owner: u16, buildings: &[EnergyBuildingPtr]) {
    for production in buildings {
        let potential_consumption = production.get_cycles_per_minute();
        let inputs = production.get_inputs();
        let buffs = production.get_buffs();
        let message = AnnoMessage {
            production_building: None,
            consumption_building: Some(ConsumptionMessage {
                address: production.address,
                island: island_name.to_owned(),
                island_id,
                island_owner,
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
