use crate::bus_ai;
use crate::dane_out::BusEvent;
use crate::dane_out::{BusId as BusID, Date, StationId as StaId, Time};
use crate::export_logs_mod;
use crate::passanger_ai;
use crate::setup_simulation;
use crate::simplestucts::*;
use crate::simulation_tick;
use std::collections::HashMap;

#[derive(Default)]
pub struct StationDataTable {
    data: HashMap<StaId, StationData>,
}
impl StationDataTable {
    pub fn insert_station_data(&mut self, data: StationData) {
        self.data.entry(data._id).or_insert(data);
    }
    pub fn get_leaving_attractivness(&self, id: &StaId, hour: u32) -> f32 {
        if hour > 23 {
            0_f32
        } else {
            self.data
                .get(id)
                .map(|entry| entry.attractivness_l[hour as usize].unwrap_or(0.5))
                .unwrap_or(0.5)
        }
    }
    pub fn get_entering_attractivness(&self, id: &StaId, hour: u32) -> f32 {
        if hour > 23 {
            0_f32
        } else {
            self.data
                .get(id)
                .map(|entry| entry.attractivness_e[hour as usize].unwrap_or(0.5))
                .unwrap_or(0.5)
        }
    }
}
#[derive(Default)]
pub struct BusLinesTable {
    data: HashMap<BusID, HashMap<StaId, Vec<Time>>>,
}
impl BusLinesTable {
    pub fn insert_station_time(&mut self, bus_id: &BusID, staid: &StaId, time: Time) {
        let data = self
            .data
            .entry(*bus_id)
            .or_default()
            .entry(*staid)
            .or_default();
        data.push(time);
        data.sort();
    }
    pub fn get_first_future_station_time(
        &self,
        bus_id: &BusID,
        current_staion: Option<&StaId>,
        start_time: Time,
    ) -> Vec<(StaId, Time)> {
        self.data
            .get(bus_id)
            .map(|data| {
                data.iter()
                    .filter_map(|(station, times)| {
                        if current_staion.is_some_and(|s| *s == *station) {
                            return None;
                        }
                        times
                            .iter()
                            .find(|t| **t >= start_time)
                            .map(|first_time| (*station, *first_time))
                    })
                    .collect::<Vec<(StaId, Time)>>()
            })
            .unwrap_or_default()
    }
}

// type StationDataTable = HashMap<StaId, StationData>;
// type BusLinesTable = HashMap<BusID, HashMap<StaId, Vec<Time>>>;
pub type BusQueueTable = Vec<(Time, BusID, StaId)>;
pub type PassangersInBusList = HashMap<BusID, Vec<Passanger>>;

const MAX_PASSANGERS_IN_BUS: u32 = 65;
const AVRG_PASSANGERS_ON_STATION: (u32, u32) = (2, 10);

pub fn run_symulacja() {
    let siminput = std::fs::read_to_string("siminput.json").expect("Error, no siminput.json file");

    export_logs_mod::setup_paths();

    let mut simulinput: SimulationInput = serde_json::from_str(&siminput).unwrap();
    setup_simulation::update_busy(&mut simulinput);

    let mut export_logs = vec![];
    let mut bus_line_logs: HashMap<BusID, Vec<(BusEvent, u32)>> = HashMap::new();

    let mut current_time: Time = simulinput.start_time;
    let mut current_date: Date = simulinput.start_date;

    let mut passangers_in_bus_list = PassangersInBusList::new();
    let mut stations = StationDataTable::default();
    let mut bus_lines = BusLinesTable::default();
    let mut bus_queue: BusQueueTable = vec![];
    setup_simulation::load_table_data(
        &mut simulinput,
        &mut stations,
        &mut bus_lines,
        &mut bus_queue,
    );
    let mut today_bus_queue = bus_queue.clone();

    loop {
        if current_date >= simulinput.end_date {
            println!("Simulation terminated on {:?}", current_date);
            break;
        }

        today_bus_queue = simulation_tick::get_today_bus_quere(&current_time, today_bus_queue);
        let next_event_time = simulation_tick::get_next_event_time(&current_time, &today_bus_queue);

        if next_event_time.is_none() {
            (current_time, current_date, today_bus_queue) =
                simulation_tick::tick_next_day(current_date, &bus_queue);
            continue;
        }

        current_time = next_event_time.unwrap();
        let events_to_simulate =
            simulation_tick::get_events_to_simulate(&current_time, &today_bus_queue);

        for (arrival_time, id_bus, id_station) in events_to_simulate {
            current_time = arrival_time;
            let (passangers_leaving, passangers_staying) =
                passanger_ai::get_passangers_leaving_amount_staying(
                    &id_bus,
                    &id_station,
                    &passangers_in_bus_list,
                );

            let current_at_e = stations.get_entering_attractivness(&id_station, current_time.hour);

            let amount_passangers_entering;
            {
                let min_passangers_enter =
                    (AVRG_PASSANGERS_ON_STATION.0 as f32 * current_at_e) as u32;
                let max_passangers_enter =
                    (AVRG_PASSANGERS_ON_STATION.1 as f32 * current_at_e) as u32;
                let max_passangers_that_can_enter_bus =
                    MAX_PASSANGERS_IN_BUS - passangers_staying.len() as u32;
                amount_passangers_entering =
                    rand::random_range(min_passangers_enter..=max_passangers_enter)
                        .min(max_passangers_that_can_enter_bus);
            }

            let mut passangers_leaving_at_station =
                bus_ai::get_passangers_leaving_at_station(&passangers_staying);

            let passangers_entering = (0..=amount_passangers_entering)
                .filter_map(|_| {
                    let posible_stations = bus_ai::get_possible_future_stations(
                        &current_time,
                        &id_bus,
                        &id_station,
                        &bus_lines,
                        &stations,
                    );
                    let posible_stations = passanger_ai::select_posible_stations(
                        posible_stations,
                        &passangers_leaving_at_station,
                        20,
                    );
                    if posible_stations.is_empty() {
                        return None;
                    }

                    let random_station = passanger_ai::select_station_to_leave(
                        &posible_stations,
                        &mut passangers_leaving_at_station,
                    );

                    Some(Passanger::new(random_station))
                })
                .collect::<Vec<_>>();

            let passangers_entering_amount = passangers_entering.len() as u32;

            bus_ai::update_passangers_in_bus(
                &id_bus,
                passangers_staying,
                passangers_entering,
                &mut passangers_in_bus_list,
            );
            let passanger_still_in_debug =
                bus_ai::get_passangers_in_bus_amount(&id_bus, &passangers_in_bus_list);
            let (export_log, bus_log) = export_logs_mod::get_export_data(
                &current_time,
                &current_date,
                &id_bus,
                &id_station,
                passangers_entering_amount,
                passangers_leaving,
                passanger_still_in_debug,
            );
            bus_line_logs
                .entry(bus_log.0.id_of_the_bus)
                .or_default()
                .push(bus_log);
            export_logs.push(export_log);
        }
        current_time.next_minute(Some(&mut current_date));
    }

    export_logs_mod::export_logs(&export_logs);
    export_logs_mod::export_bus_line_logs(&simulinput, &bus_line_logs);
}
