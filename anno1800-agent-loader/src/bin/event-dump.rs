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
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, Gauge, Paragraph, Row, Table},
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
        let island = building.get("island").and_then(Value::as_str).unwrap_or("unknown").to_string();
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
            let production = building.get("potential_production").and_then(Value::as_f64).unwrap_or(0.0);

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
        let island = building.get("island").and_then(Value::as_str).unwrap_or("unknown").to_string();
        let demand = building.get("potential_consumption").and_then(Value::as_f64).unwrap_or(0.0);
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
        let island = residence.get("island").and_then(Value::as_str).unwrap_or("unknown").to_string();
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
    product_names.get(&product_type).cloned().unwrap_or_else(|| format!("product {product_type}"))
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
                vec![Row::new(["waiting for events", "", ""])],
                [Constraint::Min(18), Constraint::Length(18), Constraint::Fill(1)],
            )
            .header(table_header())
            .block(
                Block::default()
                    .title(format!("Anno 1800 Events - latest {MAX_LOGS} raw events in {LOG_PATH}"))
                    .borders(Borders::ALL),
            );

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

            frame.render_widget(island_block, *area);
            render_resource_table(frame, table_areas[0], "Residential Consumption", residential);
            render_resource_table(frame, table_areas[1], "Production / Other", rest);
        }
    })?;

    Ok(())
}

fn table_header() -> Row<'static> {
    Row::new(["Resource", "Supply/Demand", "Balance"]).style(Style::default().add_modifier(Modifier::BOLD))
}

fn render_resource_table(frame: &mut ratatui::Frame<'_>, area: Rect, title: &'static str, resources: ResourceTotals) {
    let block = Block::default().title(title).borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = sorted_resource_rows(resources);
    let row_areas = Layout::vertical(std::iter::repeat(Constraint::Length(1)).take(rows.len() + 1)).split(inner);

    render_resource_row(frame, row_areas[0], "Resource".into(), "Supply/Demand".into(), None, true);

    for ((resource, supply, demand), area) in rows.into_iter().zip(row_areas.iter().skip(1)) {
        render_resource_row(
            frame,
            *area,
            resource.into(),
            format!("{supply:.2} / {demand:.2}").into(),
            Some((supply, demand)),
            false,
        );
    }
}

fn render_resource_row(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    resource: Span<'static>,
    supply_demand: Span<'static>,
    balance: Option<(f64, f64)>,
    header: bool,
) {
    let columns = Layout::horizontal([Constraint::Min(18), Constraint::Length(18), Constraint::Fill(1)]).split(area);
    let style = if header {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    frame.render_widget(Paragraph::new(resource).style(style), columns[0]);
    frame.render_widget(Paragraph::new(supply_demand).style(style), columns[1]);

    if header {
        frame.render_widget(Paragraph::new("Balance").style(style), columns[2]);
    } else if let Some((supply, demand)) = balance {
        frame.render_widget(balance_gauge(supply, demand), columns[2]);
    }
}

fn sorted_resource_rows(resources: ResourceTotals) -> Vec<(String, f64, f64)> {
    if resources.is_empty() {
        return Vec::new();
    }

    let mut resources = resources.into_iter().collect::<Vec<_>>();
    resources.sort_by(|(left_name, (left_supply, left_demand)), (right_name, (right_supply, right_demand))| {
        let left_ratio = fulfillment_ratio(*left_supply, *left_demand);
        let right_ratio = fulfillment_ratio(*right_supply, *right_demand);
        left_ratio.total_cmp(&right_ratio).then_with(|| left_name.cmp(right_name))
    });

    resources.into_iter().map(|(resource, (supply, demand))| (resource, supply, demand)).collect()
}

fn balance_gauge(supply: f64, demand: f64) -> Gauge<'static> {
    let ratio = fulfillment_ratio(supply, demand);
    let color = if ratio >= 1.0 { Color::Green } else { Color::Yellow };

    Gauge::default().ratio(ratio).label(Span::raw("")).gauge_style(Style::default().fg(color))
}

fn fulfillment_ratio(supply: f64, demand: f64) -> f64 {
    if demand <= 0.0 {
        return 1.0;
    }

    (supply / demand).clamp(0.0, 1.0)
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

fn add_resource(resources: &mut ResourceTotals, resource: &str, supply: f64, demand: f64) {
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
