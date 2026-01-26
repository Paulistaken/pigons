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
#[derive(Default)]
pub struct BusLinesTable {
    data: HashMap<BusID, HashMap<StaId, Vec<Time>>>,
}

pub type BusQueue = Vec<(Time, BusID, StaId)>;
pub type PassangersInBusTable = HashMap<BusID, Vec<Passanger>>;
pub type ExportEventLogs = Vec<(String, BusEvent)>;
pub type ExportBusLineLogs = HashMap<BusID, Vec<(BusEvent, u32)>>;

pub const MAX_PASSANGERS_IN_BUS: u32 = 65;
pub const AVRG_PASSANGERS_ON_STATION: (u32, u32) = (2, 10);

pub enum SimulationStateResult {
    TickContinue,
    NextDay,
    SimulationOver,
}

pub struct SimulationState {
    simulation_input: SimulationInput,

    event_export_logs: ExportEventLogs,
    bus_line_logs: ExportBusLineLogs,

    current_time: Time,
    current_date: Date,

    every_day_bus_queue: BusQueue,

    passangers_in_bus: PassangersInBusTable,
    station_data: StationDataTable,
    bus_line_data: BusLinesTable,
}

impl SimulationState {
    pub fn run_simulation(&mut self) {
        loop {
            match self.simulation_tick() {
                SimulationStateResult::TickContinue => {
                    self.current_time.next_minute(Some(&mut self.current_date));
                }
                SimulationStateResult::NextDay => {
                    self.current_date.next_day();
                    self.current_time = Time { hour: 0, minute: 0 };
                    self.passangers_in_bus.clear();
                }
                SimulationStateResult::SimulationOver => {
                    export_logs_mod::export_logs(&self.event_export_logs);
                    export_logs_mod::export_bus_line_logs(
                        &self.simulation_input,
                        &self.bus_line_logs,
                    );
                    return;
                }
            }
        }
    }
}

impl SimulationState {
    fn simulation_tick(&mut self) -> SimulationStateResult {
        if self.current_date > self.simulation_input.end_date {
            return SimulationStateResult::SimulationOver;
        }
        {
            let next_event_time = self
                .every_day_bus_queue
                .iter()
                .find(|(event_time, _, _)| *event_time >= self.current_time)
                .map(|(t, _, _)| *t);
            match next_event_time {
                Some(time) => {
                    self.current_time = time;
                }
                None => {
                    return SimulationStateResult::NextDay;
                }
            }
        }
        let events_to_simulate = self
            .every_day_bus_queue
            .iter()
            .filter(|(t, _, _)| *t == self.current_time)
            .cloned()
            .collect::<Vec<_>>();
        for (_, bus, station) in events_to_simulate {
            self.simulate_bus(bus, station);
        }
        SimulationStateResult::TickContinue
    }
}

impl SimulationState {
    fn simulate_bus(&mut self, id_bus: BusID, id_station: StaId) {
        let (passangers_leaving, passangers_staying) =
            passanger_ai::get_passangers_leaving_amount_staying(
                &id_bus,
                &id_station,
                &self.passangers_in_bus,
            );

        let amount_passangers_entering =
            self.get_passangers_coming_in_amount(&passangers_staying, &id_station);

        let mut count_passanger_destinations =
            bus_ai::get_passanger_destinations_count(&passangers_staying);

        let passangers_entering = (0..=amount_passangers_entering)
            .filter_map(|_| {
                self.simulate_passanger_coming_in(
                    &id_bus,
                    &id_station,
                    &mut count_passanger_destinations,
                )
            })
            .collect::<Vec<_>>();

        let passangers_entering_amount = passangers_entering.len() as u32;

        bus_ai::update_passangers_in_bus(
            &id_bus,
            passangers_staying,
            passangers_entering,
            &mut self.passangers_in_bus,
        );
        let passangers_in_bus_after =
            bus_ai::get_passangers_in_bus_amount(&id_bus, &self.passangers_in_bus);
        let (export_log, bus_log) = export_logs_mod::get_export_data(
            &self.current_time,
            &self.current_date,
            &id_bus,
            &id_station,
            passangers_entering_amount,
            passangers_leaving,
            passangers_in_bus_after,
        );
        self.bus_line_logs
            .entry(bus_log.0.id_of_the_bus)
            .or_default()
            .push(bus_log);
        self.event_export_logs.push(export_log);
    }
}

impl SimulationState {
    fn get_passangers_coming_in_amount(
        &self,
        passangers_staying: &[Passanger],
        id_station: &StaId,
    ) -> u32 {
        let current_att_entering = self
            .station_data
            .get_entering_attractivness(id_station, self.current_time.hour);
        let min_passangers_enter =
            (AVRG_PASSANGERS_ON_STATION.0 as f32 * current_att_entering) as u32;
        let max_passangers_enter =
            (AVRG_PASSANGERS_ON_STATION.1 as f32 * current_att_entering) as u32;
        let max_passangers_that_can_enter_bus =
            MAX_PASSANGERS_IN_BUS - passangers_staying.len() as u32;
        rand::random_range(min_passangers_enter..=max_passangers_enter)
            .min(max_passangers_that_can_enter_bus)
    }
    fn simulate_passanger_coming_in(
        &self,
        id_bus: &BusID,
        id_station: &StaId,
        passangers_leaving_at_station: &mut HashMap<StaId, u32>,
    ) -> Option<Passanger> {
        let posible_stations = bus_ai::get_possible_future_stations(
            &self.current_time,
            id_bus,
            id_station,
            &self.bus_line_data,
            &self.station_data,
        );
        let posible_stations = passanger_ai::select_posible_stations(
            posible_stations,
            passangers_leaving_at_station,
            20,
        );
        if posible_stations.is_empty() {
            return None;
        }

        let random_station =
            passanger_ai::select_station_to_leave(&posible_stations, passangers_leaving_at_station);

        Some(Passanger::new(random_station))
    }
}

impl SimulationState {
    pub fn new(mut simulatin_input_data: SimulationInput) -> Self {
        export_logs_mod::setup_export_log_paths();

        setup_simulation::update_bus_lines(&mut simulatin_input_data);

        let event_export_logs = ExportEventLogs::new();
        let bus_line_logs = ExportBusLineLogs::new();

        let current_time: Time = simulatin_input_data.start_time;
        let current_date: Date = simulatin_input_data.start_date;
        let passangers_in_bus = PassangersInBusTable::new();
        let mut station_data = StationDataTable::default();
        let mut bus_line_data = BusLinesTable::default();

        let mut every_day_bus_queue: BusQueue = vec![];
        setup_simulation::load_simulation_input_data(
            &mut simulatin_input_data,
            &mut station_data,
            &mut bus_line_data,
            &mut every_day_bus_queue,
        );
        Self {
            simulation_input: simulatin_input_data,
            event_export_logs,
            bus_line_logs,
            current_time,
            current_date,
            every_day_bus_queue,
            passangers_in_bus,
            station_data,
            bus_line_data,
        }
    }
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
