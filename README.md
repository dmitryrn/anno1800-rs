# Anno 1800 Agent
A simple data export agent for Anno 1800.
Tested against the Steam release [18.4](https://store.steampowered.com/news/app/916440/view/4471607136529417446).

## Usage
- `cargo build --release`
- `cd .\target\release\`
- `.\anno1800-agent-loader.exe`

## Export Examples

### Production Buildings

#### Fish Oil Factory
```json
{
    "production_building": {
        "address": 1966166155336,
        "island": "Pedra Seca",
        "island_id": 9540,
        "island_owner": 0,
        "product_type": 120042,
        "product_string": "Fish Oil",
        "potential_production": 2.0,
        "potential_extra_production": [],
        "inputs": []
    },
    "consumption_building": null,
    "residence_consumption": null,
    "trade_route": null,
    "trade_contracts": null
}
```

#### Mezcal Bar
```json
{
    "production_building": {
        "address": 1966151545560,
        "island": "Manola",
        "island_id": 8708,
        "island_owner": 0,
        "product_type": 6600,
        "product_string": "Mezcal",
        "potential_production": 4.0,
        "potential_extra_production": [],
        "inputs": [
            {
                "product_type": 5383,
                "product_string": "Herbs",
                "multiplier": 1
            },
            {
                "product_type": 133097,
                "product_string": "Citrus",
                "multiplier": 1
            },
            {
                "product_type": 1010239,
                "product_string": "Sugar",
                "multiplier": 1
            }
        ]
    },
    "consumption_building": null,
    "residence_consumption": null,
    "trade_route": null,
    "trade_contracts": null
}
```

#### Fishery (with Captain Moby, Old Dog of the Sea)
```json
{
    "production_building": {
        "address": 1965336666936,
        "island": "Ditchwater",
        "island_id": 8451,
        "island_owner": 0,
        "product_type": 1010200,
        "product_string": "Fish",
        "potential_production": 3.9,
        "potential_extra_production": [
            {
                "product_type": 1010234,
                "product_string": "Tallow",
                "potential_production": 0.2
            },
            {
                "product_type": 1010249,
                "product_string": "Gold",
                "potential_production": 0.14285715
            }
        ],
        "inputs": []
    },
    "consumption_building": null,
    "residence_consumption": null,
    "trade_route": null,
    "trade_contracts": null
}
```

### Consumption Buildings

#### Silo
```json
{
    "production_building": null,
    "consumption_building": {
        "address": 1966149374064,
        "island": "La Isla",
        "island_id": 8388,
        "island_owner": 0,
        "potential_consumption": 0.2,
        "potential_extra_production": [],
        "inputs": [
            120034
        ]
    },
    "residence_consumption": null,
    "trade_route": null,
    "trade_contracts": null
}
```

### Residence Consumption
```json
{
    "production_building": null,
    "consumption_building": null,
    "residence_consumption": {
        "island": "King William Island",
        "island_id": 11334,
        "island_owner": 0,
        "consumptions": [
            {
                "product_type": 535,
                "product_string": "Local Mail",
                "consumption": 2.5350006
            },
            {
                "product_type": 536,
                "product_string": "Regional Mail",
                "consumption": 0.0
            },
            {
                "product_type": 2524,
                "product_string": "Overseas Mail",
                "consumption": 0.0
            },
            {
                "product_type": 5390,
                "product_string": "Motor",
                "consumption": 0.0
            },
            {
                "product_type": 6600,
                "product_string": "Mezcal",
                "consumption": 0.0
            },
            {
                "product_type": 25506,
                "product_string": "Hot Sauce",
                "consumption": 0.0
            },
            {
                "product_type": 112700,
                "product_string": "Parkas",
                "consumption": 1.1519998
            },
            {
                "product_type": 112701,
                "product_string": "Sleeping Bags",
                "consumption": 1.5210017
            },
            {
                "product_type": 112702,
                "product_string": "Oil Lamps",
                "consumption": 1.0139991
            },
            {
                "product_type": 112703,
                "product_string": "Husky Sleds",
                "consumption": 0.8640005
            },
            {
                "product_type": 112705,
                "product_string": "Pemmican",
                "consumption": 2.0279982
            },
            {
                "product_type": 120032,
                "product_string": "Coffee",
                "consumption": 1.1519998
            },
            {
                "product_type": 1010213,
                "product_string": "Bread",
                "consumption": 0.0
            },
            {
                "product_type": 1010216,
                "product_string": "Schnapps",
                "consumption": 2.5350006
            },
            {
                "product_type": 1010217,
                "product_string": "Canned Food",
                "consumption": 0.6335999
            },
            {
                "product_type": 1010222,
                "product_string": "Dynamite",
                "consumption": 0.0
            },
            {
                "product_type": 1010234,
                "product_string": "Tallow",
                "consumption": 0.0
            },
            {
                "product_type": 1010257,
                "product_string": "Rum",
                "consumption": 0.0
            }
        ]
    },
    "trade_route": null,
    "trade_contracts": null
}
```

### Trade Routes
```json
{
    "production_building": null,
    "consumption_building": null,
    "residence_consumption": null,
    "trade_route": {
        "address": 1964088553696,
        "name": "Port Defiant",
        "owner_id": 0,
        "stops": [
            {
                "island_id": 11590,
                "slots": [
                    {
                        "product_type": 112696,
                        "product_string": "Seal Skin",
                        "amount": 50,
                        "action": 1
                    },
                    {
                        "product_type": 112705,
                        "product_string": "Pemmican",
                        "amount": 50,
                        "action": 0
                    }
                ]
            },
            {
                "island_id": 11334,
                "slots": [
                    {
                        "product_type": 112696,
                        "product_string": "Seal Skin",
                        "amount": 50,
                        "action": 0
                    },
                    {
                        "product_type": 112705,
                        "product_string": "Pemmican",
                        "amount": 50,
                        "action": 1
                    }
                ]
            }
        ]
    },
    "trade_contracts": null
}
```

### Trade Contracts
```json
{
    "production_building": null,
    "consumption_building": null,
    "residence_consumption": null,
    "trade_route": null,
    "trade_contracts": {
        "island_id": 8451,
        "contracts": [
            {
                "export_product_type": 1010216,
                "export_product_string": "Schnapps",
                "export_amount": 50,
                "import_product_type": 1010227,
                "import_product_string": "Iron",
                "import_amount": 82
            },
            {
                "export_product_type": 1010196,
                "export_product_string": "Timber",
                "export_amount": 250,
                "import_product_type": 1010227,
                "import_product_string": "Iron",
                "import_amount": 282
            },
            {
                "export_product_type": 1010213,
                "export_product_string": "Bread",
                "export_amount": 100,
                "import_product_type": 1010227,
                "import_product_string": "Iron",
                "import_amount": 261
            },
            {
                "export_product_type": 1010197,
                "export_product_string": "Wool",
                "export_amount": 500,
                "import_product_type": 1010222,
                "import_product_string": "Dynamite",
                "import_amount": 16
            },
            {
                "export_product_type": 1010198,
                "export_product_string": "Red Peppers",
                "export_amount": 500,
                "import_product_type": 1010222,
                "import_product_string": "Dynamite",
                "import_amount": 86
            }
        ]
    }
}
```
