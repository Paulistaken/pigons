use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::dane_out::{BusEvent, TDF};

type Time = crate::dane_out::Time;
type Date = crate::dane_out::Date;
type BusID = crate::dane_out::BusId;
type StaId = crate::dane_out::StationId;

#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SimuBusPlan {
    busid: BusID,
    points: Vec<SimuStopPoint>,
}
#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SimuStopPoint {
    time: Time,
    staid: StaId,
}
#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SimulationInput {
    start_time: Time,
    start_date: Date,
    bus_plans: Vec<SimuBusPlan>,
}
#[derive(Debug, Clone)]
pub struct Passanger {
    station_to_leave: StaId,
}
impl Passanger {
    pub fn new(id: StaId) -> Self {
        Self {
            station_to_leave : id
        }
    }
}

pub fn run_symulacja() {
    let siminput = std::fs::read_to_string("siminput.json").expect("Error, no siminput.json file");

    let mut simulinput: SimulationInput = serde_json::from_str(&siminput).unwrap();

    {
        let mut populating_busses_data = vec![];

        for busp in simulinput.bus_plans.iter_mut() {
            let mut bbs = busp.points.clone();
            for i in 7..23 {
                for b in bbs.iter_mut() {
                    b.time.hour = i;
                }
                busp.points.append(&mut bbs);
            }
            let mut nbusp = busp.clone();
            nbusp.busid.id_number += 10;
            nbusp
                .points
                .iter_mut()
                .zip(busp.points.iter().rev())
                .for_each(|(a, b)| a.staid = b.staid);
            populating_busses_data.push(nbusp);
        }

        simulinput.bus_plans.append(&mut populating_busses_data);
    }

    {
        let _ = std::fs::create_dir_all("simresults/pre/csv/pr");
        let _ = std::fs::create_dir_all("simresults/pre/json/pr");
    }

    let mut current_time: Time = simulinput.start_time;
    let mut current_date: Date = simulinput.start_date;

    let mut passangers_in_bus_list: HashMap<BusID, Vec<Passanger>> = HashMap::new();
    let mut bus_lines: HashMap<BusID, HashSet<StaId>> = HashMap::new();
    let mut bus_queue: Vec<(Time, BusID, StaId)> = vec![];

    for bus_plan in simulinput.bus_plans {
        for station_point in bus_plan.points {
            if let Some(bus_plan) = bus_lines.get_mut(&bus_plan.busid) {
                bus_plan.insert(station_point.staid);
            } else {
                let mut station = HashSet::new();
                station.insert(station_point.staid);
                bus_lines.insert(bus_plan.busid, station);
            }
            bus_queue.push((station_point.time, bus_plan.busid, station_point.staid));
        }
    }
    bus_queue.sort_by(|(t1, _, _), (t2, _, _)| t1.cmp(t2));

    loop {
        if current_date.month != simulinput.start_date.month {
            println!("Simulation terminated on {:?}", current_date);
            return;
        }

        println!("Simulation: {:?} {:?}", current_time, current_date);

        let busses_to_simulate;
        {
            let next_bus = bus_queue.iter().find(|(t, _, _)| *t >= current_time);
            if next_bus.is_none() {
                current_time = Time { hour: 0, minute: 0 };
                current_date.next_day();
                passangers_in_bus_list.clear();
                continue;
            }
            let (time, _, _) = next_bus.unwrap();
            busses_to_simulate = bus_queue.iter().filter(|(t, _, _)| *t == *time);
        }
        for next_bus in busses_to_simulate {
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

            let passangers_entering = (0..rand::random_range(3..20))
                .map(|_i| {
                    let plan = bus_lines.get(id_bus).cloned().unwrap_or_default();
                    let plan = plan.into_iter().collect::<Vec<_>>();
                    let station = rand::random_range(0..plan.len());
                    let station = plan[station];
                    Passanger::new(station)
                })
                .collect::<Vec<_>>();
            let passangers_entering_amount = passangers_entering.len() as u32;

            let new_passangers_in_bus = passangers_staying.into_iter().cloned().chain(passangers_entering.into_iter()).collect::<Vec<_>>();
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
            {
                let serialized_csv = export_data.export_tdf();
                let serialized_json = serde_json::to_string(&export_data).unwrap_or("".to_string());
                let pathcsv = format!(
                    "./simresults/pre/csv/BusEVENTd{:?}.{:?}.{:?}t{:?}.{:?}b{:?}s{:?}.csv",
                    current_date.year,
                    current_date.month,
                    current_date.day,
                    current_time.hour,
                    current_time.minute,
                    id_bus.id_number,
                    id_station.id_number
                );
                let pathjson = format!(
                    "./simresults/pre/json/BusEVENTd{:?}.{:?}.{:?}t{:?}.{:?}b{:?}s{:?}.json",
                    current_date.year,
                    current_date.month,
                    current_date.day,
                    current_time.hour,
                    current_time.minute,
                    id_bus.id_number,
                    id_station.id_number
                );
                let _ = fs::write(&pathcsv, &serialized_csv);
                let _ = fs::write(&pathjson, &serialized_json);
            }

            println!(
                " Bus Id:{:?}\n Station Id:{:?}\n Passangers entering:{:?}\n Passangers leaving:{:?}\n Passangers in bus:{:?}",
                id_bus.id_number,
                id_station.id_number,
                passangers_entering_amount,
                passangers_leaving_amount,
                passanger_still_in_debug
            );
        }
        current_time.next_minute(Some(&mut current_date));
    }
}
