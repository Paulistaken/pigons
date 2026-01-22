use std::collections::HashMap;

use crate::dane_out::{BusId as BusID, Date, StationId as StaId, Time};
use crate::simplestucts::Passanger;
use crate::simulsimple::BusLinesTable;
use crate::simulsimple::PassangersInBusList;
use crate::simulsimple::StationDataTable;

pub fn get_passangers_leaving_at_station(passangers_staying: &[Passanger]) -> HashMap<StaId, u32> {
    let mut passangers_leaving_at_station: HashMap<StaId, u32> = HashMap::new();
    for passanger in passangers_staying.iter() {
        *passangers_leaving_at_station
            .entry(passanger.station_to_leave)
            .or_insert(0) += 1;
    }
    passangers_leaving_at_station
}
pub fn update_passangers_in_bus(
    id_bus: &BusID,
    mut passangers_staying: Vec<Passanger>,
    mut passangers_entering: Vec<Passanger>,
    passangers_in_bus: &mut PassangersInBusList,
) {
    let passangers_in_bus = passangers_in_bus.entry(*id_bus).or_default();
    passangers_in_bus.clear();
    passangers_in_bus.append(&mut passangers_staying);
    passangers_in_bus.append(&mut passangers_entering);
}
pub fn get_passangers_in_bus_amount(id_bus: &BusID, list: &PassangersInBusList) -> u32 {
    list.get(id_bus).map(|p| p.len() as u32).unwrap_or(0_u32)
}
pub fn get_possible_future_stations<'a>(
    current_time: &'a Time,
    id_bus: &'a BusID,
    id_station: &'a StaId,
    bus_lines: &'a BusLinesTable,
    stations: &'a StationDataTable,
) -> Vec<(StaId, Time, f32)> {
    let next_stations =
        bus_lines.get_first_future_station_time(id_bus, Some(id_station), *current_time);
    let posible_stations = next_stations.into_iter().map(|(staid, times)| {
        (
            staid,
            times,
            stations.get_leaving_attractivness(&staid, times.hour),
        )
    });
    posible_stations.collect::<Vec<_>>()
}
