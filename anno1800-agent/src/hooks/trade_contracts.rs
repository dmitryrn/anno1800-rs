use crate::{
    api::{trade_contract::TradeContractPtr, trade_contracts::TradeContractsPtr},
    hooks::{send, AnnoMessage, IslandTradeContractMessage, IslandTradeContractsMessage},
};

pub unsafe fn handle_contracts(trade_route: TradeContractsPtr) {
    let island_id = trade_route.get_island_id();
    let mut contracts = vec![];

    let contracts_list = trade_route.get_contracts_list();
    let mut current = contracts_list.get_first_box();
    let end = contracts_list.get_last_box();
    while current != end {
        let contract = TradeContractPtr { address: current };
        contracts.push(IslandTradeContractMessage {
            export_product_type: contract.get_export_product_type().into(),
            export_product_string: format!("{:?}", contract.get_export_product_type()),
            export_amount: contract.get_export_amount(),
            import_product_type: contract.get_import_product_type().into(),
            import_product_string: format!("{:?}", contract.get_import_product_type()),
            import_amount: contract.get_import_amount(),
        });
        current += 0x24;
    }

    let message = AnnoMessage {
        production_building: None,
        consumption_building: None,
        residence_consumption: None,
        trade_route: None,
        trade_contracts: Some(IslandTradeContractsMessage { island_id, contracts }),
    };
    send(&format!("{}\n", &serde_json::to_string(&message).unwrap()));
}
