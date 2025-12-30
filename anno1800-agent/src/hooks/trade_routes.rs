use crate::{
    api::trade_route::TradeRoutePtr,
    hooks::{send, AnnoMessage, TradeRouteMessage, TradeRouteStopMessage, TradeRouteStopSlotMessage},
};

pub unsafe fn handle_trade_route(trade_route: TradeRoutePtr) {
    let address = trade_route.address;
    let name = trade_route.get_route_name();
    let owner_id = trade_route.get_owner_id();
    if owner_id != 0 {
        return;
    }
    let stops = trade_route
        .get_stops()
        .iter()
        .map(|e| TradeRouteStopMessage {
            island_id: e.get_island_id(),
            slots: e
                .get_slots()
                .iter()
                .map(|e| TradeRouteStopSlotMessage {
                    action: e.get_action(),
                    amount: e.get_amount(),
                    product_type: e.get_product_type().into(),
                    product_string: format!("{:?}", e.get_product_type()),
                })
                .collect(),
        })
        .collect();
    let message = AnnoMessage {
        production_building: None,
        consumption_building: None,
        residence_consumption: None,
        trade_route: Some(TradeRouteMessage {
            address,
            name,
            owner_id,
            stops,
        }),
        trade_contracts: None,
    };
    send(&format!("{}\n", &serde_json::to_string(&message).unwrap()));
}
