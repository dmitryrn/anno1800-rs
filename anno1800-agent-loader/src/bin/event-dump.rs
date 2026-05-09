use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs;
use std::io::{self, stdout, ErrorKind, Stdout};
use std::net::UdpSocket;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Margin},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Clear, Row, Table},
    Terminal,
};
use serde_json::Value;

const LOG_PATH: &str = "recent-events.log";
const MAX_LOGS: usize = 100;
const BUILDING_TTL: Duration = Duration::from_secs(5);

type BuildingResources = Vec<(String, String, f64, f64)>;
type ProductionBuildings = HashMap<u64, (Instant, BuildingResources)>;
type ConsumptionBuildings = HashMap<u64, (Instant, BuildingResources)>;
type ResourceTotals = BTreeMap<String, (f64, f64)>;
type IslandTotals = BTreeMap<String, (ResourceTotals, ResourceTotals)>;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:1800")?;
    socket.set_read_timeout(Some(Duration::from_millis(200)))?;
    let mut buffer = [0; 65_507];
    let mut logs = VecDeque::new();
    let mut product_names = HashMap::new();
    let mut production_buildings = HashMap::new();
    let mut consumption_buildings = HashMap::new();
    let mut residence_consumptions = HashMap::new();
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    terminal.clear()?;
    let running = Arc::new(AtomicBool::new(true));
    let handler_running = Arc::clone(&running);
    ctrlc::set_handler(move || {
        handler_running.store(false, Ordering::SeqCst);
    })
    .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

    render_state(&mut terminal, &production_buildings, &consumption_buildings, &residence_consumptions)?;

    while running.load(Ordering::SeqCst) {
        let (len, from) = match socket.recv_from(&mut buffer) {
            Ok(packet) => packet,
            Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                if remove_stale_buildings(&mut production_buildings, &mut consumption_buildings) {
                    render_state(&mut terminal, &production_buildings, &consumption_buildings, &residence_consumptions)?;
                }
                continue;
            }
            Err(e) => return Err(e),
        };
        let payload = String::from_utf8_lossy(&buffer[..len]);
        let parsed = serde_json::from_str::<Value>(&payload);
        let payload = parsed
            .as_ref()
            .ok()
            .and_then(|value| serde_json::to_string_pretty(value).ok())
            .unwrap_or_else(|| payload.into_owned());
        let event = format!("--- event from {from} ({len} bytes) ---\n{payload}");

        logs.push_back(event);
        if logs.len() > MAX_LOGS {
            logs.pop_front();
        }

        fs::write(LOG_PATH, logs.iter().cloned().collect::<Vec<_>>().join("\n\n"))?;

        if let Ok(value) = parsed {
            update_state(
                &value,
                &mut product_names,
                &mut production_buildings,
                &mut consumption_buildings,
                &mut residence_consumptions,
            );
        }

        remove_stale_buildings(&mut production_buildings, &mut consumption_buildings);

        render_state(&mut terminal, &production_buildings, &consumption_buildings, &residence_consumptions)?;
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn update_state(
    event: &Value,
    product_names: &mut HashMap<u64, String>,
    production_buildings: &mut ProductionBuildings,
    consumption_buildings: &mut ConsumptionBuildings,
    residence_consumptions: &mut HashMap<String, Vec<(String, f64)>>,
) {
    if let Some(building) = event.get("production_building").and_then(Value::as_object) {
        let Some(address) = building.get("address").and_then(Value::as_u64) else {
            return;
        };
        let island = building
            .get("island")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        let mut resources = Vec::new();

        if let (Some(product_type), Some(product_string), Some(supply)) = (
            building.get("product_type").and_then(Value::as_u64),
            building.get("product_string").and_then(Value::as_str),
            building.get("potential_production").and_then(Value::as_f64),
        ) {
            product_names.insert(product_type, product_string.to_string());
            resources.push((island.clone(), product_string.to_string(), supply, 0.0));
        }

        if let Some(extras) = building.get("potential_extra_production").and_then(Value::as_array) {
            for extra in extras {
                if let (Some(product_type), Some(product_string), Some(supply)) = (
                    extra.get("product_type").and_then(Value::as_u64),
                    extra.get("product_string").and_then(Value::as_str),
                    extra.get("potential_production").and_then(Value::as_f64),
                ) {
                    product_names.insert(product_type, product_string.to_string());
                    resources.push((island.clone(), product_string.to_string(), supply, 0.0));
                }
            }
        }

        if let Some(inputs) = building.get("inputs").and_then(Value::as_array) {
            let production = building
                .get("potential_production")
                .and_then(Value::as_f64)
                .unwrap_or(0.0);

            for input in inputs {
                if let (Some(product_type), Some(product_string), Some(multiplier)) = (
                    input.get("product_type").and_then(Value::as_u64),
                    input.get("product_string").and_then(Value::as_str),
                    input.get("multiplier").and_then(Value::as_f64),
                ) {
                    product_names.insert(product_type, product_string.to_string());
                    resources.push((island.clone(), product_string.to_string(), 0.0, production * multiplier));
                }
            }
        }

        production_buildings.insert(address, (Instant::now(), resources));
    }

    if let Some(building) = event.get("consumption_building").and_then(Value::as_object) {
        let Some(address) = building.get("address").and_then(Value::as_u64) else {
            return;
        };
        let island = building
            .get("island")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        let demand = building
            .get("potential_consumption")
            .and_then(Value::as_f64)
            .unwrap_or(0.0);
        let resources = building
            .get("inputs")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_u64)
            .map(|product_type| (island.clone(), resource_name(product_type, product_names), 0.0, demand))
            .collect();

        consumption_buildings.insert(address, (Instant::now(), resources));
    }

    if let Some(residence) = event.get("residence_consumption").and_then(Value::as_object) {
        let island = residence
            .get("island")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        let resources = residence
            .get("consumptions")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|consumption| {
                let product_type = consumption.get("product_type").and_then(Value::as_u64)?;
                let product_string = consumption.get("product_string").and_then(Value::as_str)?;
                let demand = consumption.get("consumption").and_then(Value::as_f64)?;
                product_names.insert(product_type, product_string.to_string());
                Some((product_string.to_string(), demand))
            })
            .collect();

        residence_consumptions.insert(island, resources);
    }
}

fn resource_name(product_type: u64, product_names: &HashMap<u64, String>) -> String {
    product_names
        .get(&product_type)
        .cloned()
        .unwrap_or_else(|| format!("product {product_type}"))
}

fn render_state(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    production_buildings: &ProductionBuildings,
    consumption_buildings: &ConsumptionBuildings,
    residence_consumptions: &HashMap<String, Vec<(String, f64)>>,
) -> io::Result<()> {
    let state = sum_supply_demand(production_buildings, consumption_buildings, residence_consumptions);

    terminal.draw(|frame| {
        frame.render_widget(Clear, frame.area());

        if state.is_empty() {
            let table = Table::new(
                vec![Row::new(["waiting for events", ""])],
                [Constraint::Min(24), Constraint::Length(20)],
            )
            .header(table_header())
            .block(Block::default().title(format!("Anno 1800 Events - latest {MAX_LOGS} raw events in {LOG_PATH}")).borders(Borders::ALL));

            frame.render_widget(table, frame.area());
            return;
        }

        let areas = Layout::vertical(
            state
                .values()
                .map(|(residential, rest)| Constraint::Length(residential.len().max(rest.len()) as u16 + 5)),
        )
        .split(frame.area());

        for ((island, (residential, rest)), area) in state.into_iter().zip(areas.iter()) {
            let island_block = Block::default().title(island).borders(Borders::ALL);
            let inner = area.inner(Margin { horizontal: 1, vertical: 1 });
            let table_areas = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(inner);
            let residential_table = resource_table("Residential Consumption", residential);
            let rest_table = resource_table("Production / Other", rest);

            frame.render_widget(island_block, *area);
            frame.render_widget(residential_table, table_areas[0]);
            frame.render_widget(rest_table, table_areas[1]);
        }
    })?;

    Ok(())
}

fn table_header() -> Row<'static> {
    Row::new(["Resource", "Supply/Demand"]).style(Style::default().add_modifier(Modifier::BOLD))
}

fn resource_table(title: &'static str, resources: ResourceTotals) -> Table<'static> {
    Table::new(table_rows(resources), [Constraint::Min(24), Constraint::Length(20)])
        .header(table_header())
        .block(Block::default().title(title).borders(Borders::ALL))
}

fn table_rows(resources: ResourceTotals) -> Vec<Row<'static>> {
    if resources.is_empty() {
        return vec![Row::new(["", ""])];
    }

    let mut resources = resources.into_iter().collect::<Vec<_>>();
    resources.sort_by(|(left_name, (left_supply, left_demand)), (right_name, (right_supply, right_demand))| {
        let left_imbalance = (left_supply - left_demand).abs();
        let right_imbalance = (right_supply - right_demand).abs();
        right_imbalance
            .total_cmp(&left_imbalance)
            .then_with(|| left_name.cmp(right_name))
    });

    resources
        .into_iter()
        .map(|(resource, (supply, demand))| {
            Row::new(vec![
                Cell::from(resource),
                Cell::from(format!("{supply:.2} / {demand:.2}")),
            ])
        })
        .collect()
}

fn sum_supply_demand(
    production_buildings: &ProductionBuildings,
    consumption_buildings: &ConsumptionBuildings,
    residence_consumptions: &HashMap<String, Vec<(String, f64)>>,
) -> IslandTotals {
    let mut state: IslandTotals = BTreeMap::new();

    for (_, resources) in production_buildings.values() {
        for (island, resource, supply, demand) in resources {
            add_resource(&mut state.entry(island.to_string()).or_default().1, resource, *supply, *demand);
        }
    }

    for (_, resources) in consumption_buildings.values() {
        for (island, resource, supply, demand) in resources {
            add_resource(&mut state.entry(island.to_string()).or_default().1, resource, *supply, *demand);
        }
    }

    for (island, resources) in residence_consumptions {
        for (resource, demand) in resources {
            add_resource(&mut state.entry(island.to_string()).or_default().0, resource, 0.0, *demand);
        }
    }

    for (residential, rest) in state.values_mut() {
        for (resource, (residential_supply, _)) in residential.iter_mut() {
            if let Some((supply, _)) = rest.remove(resource) {
                *residential_supply += supply;
            }
        }

        residential.retain(|_, (_, demand)| *demand != 0.0);
    }

    state
}

fn add_resource(
    resources: &mut ResourceTotals,
    resource: &str,
    supply: f64,
    demand: f64,
) {
    let totals = resources.entry(resource.to_string()).or_default();
    totals.0 += supply;
    totals.1 += demand;
}

fn remove_stale_buildings(production_buildings: &mut ProductionBuildings, consumption_buildings: &mut ConsumptionBuildings) -> bool {
    let before = production_buildings.len() + consumption_buildings.len();

    production_buildings.retain(|_, (last_seen, _)| last_seen.elapsed() <= BUILDING_TTL);
    consumption_buildings.retain(|_, (last_seen, _)| last_seen.elapsed() <= BUILDING_TTL);

    before != production_buildings.len() + consumption_buildings.len()
}
