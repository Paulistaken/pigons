use std::{collections::HashMap, fs};

use rand::random_range;

use crate::dane_out::{BusEvent, StationId, TDF};

use super::simplestucts::*;

type StationDataTable = HashMap<StaId, StationData>;
type BusLinesTable = HashMap<BusID, HashMap<StaId, Vec<Time>>>;
type BusQueueTable = Vec<(Time, BusID, StaId)>;

const MAX_PASSANGERS_IN_BUS: u32 = 65;
const AVRG_PASSANGERS_ON_STATION: (u32, u32) = (2,10);
const DEBUG_PRINTS: bool = false;

pub fn run_symulacja() {
    let siminput = std::fs::read_to_string("siminput.json").expect("Error, no siminput.json file");

    let mut simulinput: SimulationInput = serde_json::from_str(&siminput).unwrap();
    update_busy(&mut simulinput);

    let mut export_logs = vec![];
    let mut bus_line_logs: HashMap<BusID, Vec<(BusEvent, u32)>> = HashMap::new();

    {
        let _ = std::fs::create_dir_all("simresults/pre/csv/pr");
        let _ = std::fs::create_dir_all("simresults/pre/json/pr");
        let _ = std::fs::create_dir_all("simresults/pre/bus_logs/");
    }

    let mut current_time: Time = simulinput.start_time;
    let mut current_date: Date = simulinput.start_date;

    let mut passangers_in_bus_list: HashMap<BusID, Vec<Passanger>> = HashMap::new();
    let mut stations: StationDataTable = StationDataTable::new();
    let mut bus_lines: BusLinesTable = BusLinesTable::new();
    let mut bus_queue: BusQueueTable = vec![];
    load_table_data(
        &mut simulinput,
        &mut stations,
        &mut bus_lines,
        &mut bus_queue,
    );

    loop {
        if current_date.month == 4 {
            println!("Simulation terminated on {:?}", current_date);
            break;
        }

        if DEBUG_PRINTS {
            println!("Simulation: {:?} {:?}", current_time, current_date);
        }

        let events_to_simulate = bus_queue.iter().filter(|(t, _, _)| *t >= current_time);
        let event_time = events_to_simulate.clone().map(|(t, _, _)| *t).min();
        if event_time.is_none() {
            current_time = Time { hour: 0, minute: 0 };
            current_date.next_day();
            passangers_in_bus_list.clear();
            continue;
        }
        let events_to_simulate = events_to_simulate
            .filter(|(t, _, _)| *t == event_time.unwrap())
            .collect::<Vec<_>>();
        for next_bus in events_to_simulate {
            let (arrival_time, id_bus, id_station) = next_bus;
            current_time = *arrival_time;
            let passangers_in_bus = passangers_in_bus_list
                .get(id_bus)
                .cloned()
                .unwrap_or(vec![]);
            let passangers_staying = passangers_in_bus
                .iter()
                .filter(|p| p.station_to_leave != *id_station)
                .collect::<Vec<_>>();

            let passangers_leaving = passangers_in_bus
                .iter()
                .filter(|p| p.station_to_leave == *id_station)
                .collect::<Vec<_>>();
            let passangers_leaving_amount = passangers_leaving.len() as u32;

            let current_att =
                stations[id_station].attractivness[current_time.hour as usize].unwrap_or(0.5);
            let min_passangers_enter = (AVRG_PASSANGERS_ON_STATION.0 as f32 * current_att) as u32;
            let max_passangers_enter = (AVRG_PASSANGERS_ON_STATION.1 as f32 * current_att) as u32;
            let max_passangers_that_can_enter_bus =
                MAX_PASSANGERS_IN_BUS - passangers_staying.len() as u32;
            let passangers_entering =
                (0..=rand::random_range(min_passangers_enter..=max_passangers_enter)
                    .min(max_passangers_that_can_enter_bus))
                    .filter_map(|_i| {
                        let posible_stations = get_possible_future_stations(
                            &current_time,
                            id_bus,
                            id_station,
                            &bus_lines,
                            &stations,
                        );
                        let posible_stations = posible_stations.into_iter().filter(|(s,_,_)|{
                            let ct = passangers_staying.iter().filter(|p| p.station_to_leave == **s).count();
                            ct < 20
                        }).collect::<Vec<_>>();
                        if posible_stations.is_empty() {
                            return None;
                        }
                        let random_station = get_random_station(&posible_stations);
                        Some(Passanger::new(random_station))
                    })
                    .collect::<Vec<_>>();
            let passangers_entering_amount = passangers_entering.len() as u32;

            let new_passangers_in_bus = passangers_staying
                .into_iter()
                .cloned()
                .chain(passangers_entering.into_iter())
                .collect::<Vec<_>>();
            let passanger_still_in_debug = new_passangers_in_bus.len();
            passangers_in_bus_list.insert(*id_bus, new_passangers_in_bus);

            let export_data = BusEvent {
                date_of_event: current_date,
                time_of_event: current_time,
                id_of_the_bus: *id_bus,
                id_of_the_station: *id_station,
                pasangers_coming_out: passangers_leaving_amount,
                pasangers_coming_in: passangers_entering_amount,
                ..Default::default()
            };
            let pathname = format!(
                "y{:?}m{:?}d{:?}h{:?}m{:?}b{:?}s{:?}",
                current_date.year,
                current_date.month,
                current_date.day,
                current_time.hour,
                current_time.minute,
                id_bus.id_number,
                id_station.id_number
            );
            bus_line_logs
                .entry(export_data.id_of_the_bus)
                .or_default()
                .push((export_data.clone(), passanger_still_in_debug as u32));
            export_logs.push((pathname, export_data));

            if DEBUG_PRINTS {
                println!(
                    " Bus Id:{:?}\n Station Id:{:?}\n Passangers entering:{:?}\n Passangers leaving:{:?}\n Passangers in bus:{:?}\n  {:?} | {:?}",
                    id_bus.id_number,
                    id_station.id_number,
                    passangers_entering_amount,
                    passangers_leaving_amount,
                    passanger_still_in_debug,
                    current_time,
                    arrival_time
                );
            }
        }
        current_time.next_minute(Some(&mut current_date));
    }
    for (bus_id, export_data) in bus_line_logs {
        let path = format!("./simresults/pre/bus_logs/bus{:?}.txt", bus_id.id_number);
        let path2 = format!("./simresults/pre/bus_logs/bus{:?}.csv", bus_id.id_number);
        let mut content = "".to_string();
        let mut content2 = "hour,minute,year,month,day,station,in,out,inside\n".to_string();
        let mut cdate = simulinput.start_date;
        for (data, pin) in export_data {
            if data.date_of_event != cdate {
                content += &format!("{:?}\n", data.date_of_event);
                cdate = data.date_of_event;
            }
            content += &format!(
                "{:?}, {:?}, People coming in {:?}, People going out {:?}, People inside the bus {:?}\n",
                data.time_of_event,
                data.id_of_the_station,
                data.pasangers_coming_in,
                data.pasangers_coming_out,
                pin
            );
            content2 += &format!("{},{},", data.time_of_event.hour, data.time_of_event.minute);
            content2 += &format!(
                "{},{},{},",
                data.date_of_event.year, data.date_of_event.month, data.date_of_event.day
            );
            content2 += &format!(
                "{},{},{},{}\n",
                data.id_of_the_station.id_number,
                data.pasangers_coming_in,
                data.pasangers_coming_out,
                pin
            );
        }
        let _ = fs::write(&path, &content);
        let _ = fs::write(&path2, &content2);
    }
    for (export_path, export_data) in export_logs {
        let serialized_csv = export_data.export_tdf();
        let serialized_csv = serialized_csv.0 + "\n" + &serialized_csv.1;
        let serialized_json = serde_json::to_string(&export_data).unwrap_or("".to_string());
        let pathcsv = "./simresults/pre/csv/BusEVENT".to_string() + &export_path + ".csv";
        let pathjson = "./simresults/pre/json/BusEVENT".to_string() + &export_path + ".json";
        let _ = fs::write(&pathcsv, &serialized_csv);
        let _ = fs::write(&pathjson, &serialized_json);
    }
}

fn load_table_data(
    simulinput: &mut SimulationInput,
    stations: &mut StationDataTable,
    bus_lines: &mut BusLinesTable,
    bus_queue: &mut BusQueueTable,
) {
    for stationdata in simulinput.stations.iter() {
        stations.insert(stationdata.staid, stationdata.clone().into());
    }
    for bus_plan in simulinput.bus_plans.iter() {
        let busp = bus_lines.entry(bus_plan.busid).or_default();
        for station_point in bus_plan.points.iter() {
            let stap = busp.entry(station_point.staid).or_default();
            stap.push(station_point.time);
            stap.sort();
            bus_queue.push((station_point.time, bus_plan.busid, station_point.staid));
        }
    }
    bus_queue.sort_by(|(t1, _, _), (t2, _, _)| t1.cmp(t2));
}
fn update_busy(simulinput: &mut SimulationInput) {
    for busp in simulinput.bus_plans.iter_mut() {
        let mut bbs = busp.points.clone();
        for i in 7..=23 {
            for b in bbs.iter_mut() {
                b.time.hour = i;
            }
            busp.points.append(&mut bbs.clone());
        }
    }
}
fn get_random_station<'a, 'b: 'a>(stations: &'b [(&'a StaId, &'a Time, Option<f32>)]) -> StaId {
    let mut stations = stations.to_vec();
    stations.sort_by(|(_, _, at1), (_, _, at2)| {
        let at1 = at1.unwrap_or(0.5);
        let at2 = at2.unwrap_or(0.5);
        at1.total_cmp(&at2)
    });
    stations.reverse();
    let allp = stations
        .iter()
        .fold(0_f32, |a, (_, _, v)| a + v.unwrap_or(0.5));
    let rand = random_range(0_f32..allp);
    let mut p = 0_f32;
    for (staid, _, at) in stations.iter() {
        p += at.unwrap_or(0.5);
        if p >= rand {
            return **staid;
        }
    }
    *stations[0].0
}
fn get_possible_future_stations<'a>(
    current_time: &'a Time,
    id_bus: &'a BusID,
    id_station: &'a StaId,
    bus_lines: &'a HashMap<BusID, HashMap<StaId, Vec<Time>>>,
    stations: &'a HashMap<StationId, StationData>,
) -> Vec<(&'a StaId, &'a Time, Option<f32>)> {
    let bus_stations = bus_lines.get(id_bus).unwrap();
    let stations_without_current = bus_stations
        .iter()
        .filter(|(staid, _)| **staid != *id_station);
    let stations_possible_in_time = stations_without_current.filter_map(|(staid, times)| {
        times
            .iter()
            .find(|t| {
                t.hour > current_time.hour
                    || (t.hour == current_time.hour && t.minute > current_time.minute)
            })
            .map(|times| (staid, times))
    });
    let posible_stations = stations_possible_in_time.map(|(staid, times)| {
        (
            staid,
            times,
            stations.get(staid).unwrap().attractivness[times.hour as usize],
        )
    });
    posible_stations.collect::<Vec<_>>()
}
