use std::collections::HashMap;

use crate::dane_out::{BusId as BusID, StationId as StaId, Time};
use crate::simplestucts::Passanger;
use crate::simulsimple::PassangersInBusTable;
pub fn select_posible_stations(
    posible_stations: Vec<(StaId, Time, f32)>,
    people_leaving_at_station: &HashMap<StaId, u32>,
    max_people_leaving_at_station: u32,
) -> Vec<(StaId, Time, f32)> {
    posible_stations
        .into_iter()
        .filter(|(s, _, _)| {
            people_leaving_at_station.get(s).cloned().unwrap_or(0) < max_people_leaving_at_station
        })
        .collect::<Vec<_>>()
}
pub fn select_station_to_leave(
    stations: &[(StaId, Time, f32)],
    passangers_leaving_at_station: &mut HashMap<StaId, u32>,
) -> StaId {
    let mut stations = stations.to_vec();
    stations.sort_by(|(_, _, at1), (_, _, at2)| at1.total_cmp(at2));
    stations.reverse();
    let allp = stations.iter().fold(0_f32, |a, (_, _, v)| a + v);
    let rand = rand::random_range(0_f32..allp);
    let mut p = 0_f32;
    for (staid, _, at) in stations.iter() {
        p += at;
        if p >= rand {
            *passangers_leaving_at_station.entry(*staid).or_insert(0) += 1;
            return *staid;
        }
    }
    *passangers_leaving_at_station
        .entry(stations[0].0)
        .or_insert(0) += 1;
    stations[0].0
}
pub fn get_passangers_leaving_amount_staying(
    id_bus: &BusID,
    id_station: &StaId,
    passangers_in_bus: &PassangersInBusTable,
) -> (u32, Vec<Passanger>) {
    if let Some(data) = passangers_in_bus.get(id_bus) {
        let passangers_in_bus = data.len();
        let passangers_staying = data
            .iter()
            .filter(|p| p.station_to_leave != *id_station)
            .cloned()
            .collect::<Vec<_>>();
        let passangers_leaving = passangers_in_bus - passangers_staying.len();
        (passangers_leaving as u32, passangers_staying)
    } else {
        (0, vec![])
    }
}
