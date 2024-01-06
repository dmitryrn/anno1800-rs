use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::api::{class46::Class46Ptr, ware_type::WareType, AnnoPtr};

use super::send;

#[derive(Serialize, Deserialize)]
struct ProductionMessage {
    address: u64,
    island: u64,
    ware_type: u32,
    potential_production: f32,
    potential_extra_production: Vec<ExtraProductionMessage>,
}

#[derive(Serialize, Deserialize)]
struct ExtraProductionMessage {
    ware_type: u32,
    potential_production: f32,
}

pub unsafe fn handle_demand2_loop(class46_ptr: u64, weird_id: u32) {
    let class46 = Class46Ptr::new(class46_ptr);
    let class20 = class46.get_class20(weird_id);
    let production_buildings = class20.get_production_buildings();
    send(&format!(
        "class46={:#018x} class20={:#018x} {} buildings\n",
        class46.address,
        class20.address,
        production_buildings.len()
    ));
    return;
    for production_building in production_buildings {
        let building_type = production_building.get_ware_type();
        let potential_production = production_building.get_prod_thingy().get_class34(&building_type).get_potential_production();
        let buffs = production_building.get_buffs();
        let message = ProductionMessage {
            address: production_building.address,
            island: class20.address,
            ware_type: building_type.into(),
            potential_production,
            potential_extra_production: buffs
                .iter()
                .map(|e| ExtraProductionMessage {
                    ware_type: e.get_building_type().into(),
                    potential_production: e.get_value(),
                })
                .collect(),
        };
        send(&format!("{}\n", &serde_json::to_string(&message).unwrap()));
    }
}

pub unsafe fn handle_demand2_loop_old(class46_ptr: u64, weird_id: u32) {
    let class46 = Class46Ptr::new(class46_ptr);
    let class20 = class46.get_class20(weird_id);
    // send(&format!("handle_demand2_loop {:?}\n", class20));
    let production_buildings = class20.get_production_buildings();
    let mut buf = format!(
        "handle_demand2_loop class20={:#018x} buildings={}\n",
        class20.address,
        production_buildings.len()
    );
    let mut map: BTreeMap<WareType, (usize, f32)> = BTreeMap::new();
    for production_building in production_buildings {
        let building_type = production_building.get_ware_type();
        let prod_factor = production_building.get_potential_productivity_factor();
        let potential_production = production_building.get_prod_thingy().get_class34(&building_type).get_potential_production();
        let buffs = production_building.get_buffs();
        map.entry(building_type).or_default().0 += 1;
        map.entry(building_type).or_default().1 += potential_production;
        buf.push_str(&format!(
            "    {:#018x} {:<30?} ({:.02}/min, prod_factor={:.02}) {:?}\n",
            production_building.address, building_type, potential_production, prod_factor, &buffs
        ))
    }
    for (key, val) in map.iter() {
        buf.push_str(&format!("    {:<30?} {:4} Buildings, {:6.02}t/min\n", key, val.0, val.1))
    }
    if class20.address == 0x000002856a988bd0 {
        send(&buf);
    }
}
