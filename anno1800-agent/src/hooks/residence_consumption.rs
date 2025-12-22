use crate::api::area_residence_consumption_manager::AreaResidenceConsumptionManagerPtr;

use super::{send, AnnoMessage, ResidenceConsumptionMessage, ResidenceConsumptionsMessage};

pub unsafe fn handle_residences(arcm: AreaResidenceConsumptionManagerPtr) {
    let island_name = arcm.get_island().get_custom_name().get_string();
    let mut consumptions = vec![];
    let class30s = arcm.get_class30s();
    for class30 in class30s {
        consumptions.push(ResidenceConsumptionMessage {
            consumption: 60.0 * class30.get_demand_per_second(),
            ware_type: class30.get_ware_type().into(),
            ware_string: format!("{:?}", class30.get_ware_type()),
        });
    }

    let message = AnnoMessage {
        production_building: None,
        consumption_building: None,
        residence_consumption: Some(ResidenceConsumptionsMessage {
            island: island_name,
            consumptions,
        }),
    };
    send(&format!("{}\n", &serde_json::to_string(&message).unwrap()));
}
