
use crate::simplestucts::SimulationInput;
use crate::simulsimple::BusLinesTable;
use crate::simulsimple::BusQueueTable;
use crate::simulsimple::StationDataTable;
pub fn load_table_data(
    simulinput: &mut SimulationInput,
    stations: &mut StationDataTable,
    bus_lines: &mut BusLinesTable,
    bus_queue: &mut BusQueueTable,
) {
    for stationdata in simulinput.stations.iter() {
        stations.insert_station_data(stationdata.clone().into());
    }
    for bus_plan in simulinput.bus_plans.iter() {
        for station_point in bus_plan.points.iter() {
            bus_lines.insert_station_time(
                &bus_plan.busid,
                &station_point.staid,
                station_point.time,
            );
            bus_queue.push((station_point.time, bus_plan.busid, station_point.staid));
        }
    }
    bus_queue.sort_by(|(t1, _, _), (t2, _, _)| t1.cmp(t2));
}
pub fn update_busy(simulinput: &mut SimulationInput) {
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
