use crate::dane_out::{BusId as BusID, Date, StationId as StaId, Time};
use crate::simulsimple::BusQueueTable;

pub fn tick_next_day(
    mut simulation_date: Date,
    bus_queue: &BusQueueTable,
) -> (Time, Date, BusQueueTable) {
    simulation_date.next_day();
    (
        Time { hour: 0, minute: 0 },
        simulation_date,
        bus_queue.clone(),
    )
}
pub fn get_today_bus_quere(
    simulation_time: &Time,
    today_bus_queue: BusQueueTable,
) -> BusQueueTable {
    today_bus_queue
        .into_iter()
        .filter(|(t, _, _)| *t >= *simulation_time)
        .collect::<Vec<_>>()
}
pub fn get_next_event_time<'a>(
    simulation_time: &'a Time,
    today_bus_queue: &'a BusQueueTable,
) -> Option<Time> {
    today_bus_queue
        .iter()
        .find(|(t, _, _)| *t >= *simulation_time)
        .map(|(t, _, _)| *t)
}
pub fn get_events_to_simulate<'a>(
    simulation_time: &'a Time,
    today_bus_queue: &'a BusQueueTable,
) -> Vec<(Time, BusID, StaId)> {
    today_bus_queue
        .iter()
        .filter(|(t, _, _)| *t == *simulation_time)
        .map(|(t, a, b)| (*t, *a, *b))
        .collect::<Vec<_>>()
}
