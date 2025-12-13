use std::{collections::HashMap, fs};

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

pub fn run_symulacja() {
    let siminput = std::fs::read_to_string("siminput.json").unwrap();
    let mut simulinput: SimulationInput = serde_json::from_str(&siminput).unwrap();
    let mut newbbs = vec![];
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
        newbbs.push(nbusp);
    }
    simulinput.bus_plans.append(&mut newbbs);
    // let mut exampleinput = SimulationInput::default();
    // exampleinput.bus_plans.push(SimuBusPlan {
    //     busid: BusID::default(),
    //     points: vec![
    //         SimuStopPoint {
    //             time: Time::default(),
    //             staid: StaId::default(),
    //         },
    //         SimuStopPoint {
    //             time: Time::default(),
    //             staid: StaId::default(),
    //         },
    //     ],
    // });
    // exampleinput.bus_plans.push(SimuBusPlan {
    //     busid: BusID::default(),
    //     points: vec![
    //         SimuStopPoint {
    //             time: Time::default(),
    //             staid: StaId::default(),
    //         },
    //         SimuStopPoint {
    //             time: Time::default(),
    //             staid: StaId::default(),
    //         },
    //         SimuStopPoint {
    //             time: Time::default(),
    //             staid: StaId::default(),
    //         },
    //     ],
    // });
    // let out = serde_json::to_string(&exampleinput).unwrap_or("".to_string());
    // let _ = std::fs::write("siminput.json", &out);

    let mut current_time: Time = simulinput.start_time;
    let mut current_date: Date = simulinput.start_date;
    let mut pass_in_bus: HashMap<BusID, u32> = HashMap::new();
    let mut planybus: Vec<(Time, BusID, StaId)> = vec![];
    for bp in simulinput.bus_plans {
        for p in bp.points {
            planybus.push((p.time, bp.busid, p.staid));
        }
    }
    planybus.sort_by(|(t1, _, _), (t2, _, _)| t1.cmp(t2));

    // let mut maxiter = 0_u32;
    loop {
        if current_date.month != simulinput.start_date.month {
            return;
        }
        // maxiter += 1;
        // if maxiter > 100 {
        //     return;
        // }
        println!("Simulation: {:?} {:?}", current_time, current_date);
        let next_bus = planybus.iter().find(|(t, _, _)| *t >= current_time);
        if next_bus.is_none() {
            current_time = Time { hour: 0, minute: 0 };
            current_date.next_day();
            continue;
        }
        let (time, _, _) = next_bus.unwrap();
        let next_buss = planybus.iter().filter(|(t, _, _)| *t == *time);
        for next_bus in next_buss {
            let (time, busid, staid) = next_bus;
            current_time = *time;
            let passinbus = pass_in_bus.get(busid).cloned().unwrap_or(0);
            let pass_to_leave = rand::random_range(0..=passinbus);
            let pass_to_enter = rand::random_range(0..=10);
            let newbus = passinbus - pass_to_leave + pass_to_enter;

            let export_data = BusEvent {
                date_of_event: current_date,
                time_of_event: current_time,
                id_of_the_bus: *busid,
                id_of_the_station: *staid,
                pasangers_coming_out: pass_to_leave,
                pasangers_coming_in: pass_to_enter,
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
                    busid.id_number,
                    staid.id_number
                );
                let pathjson = format!(
                    "./simresults/pre/json/BusEVENTd{:?}.{:?}.{:?}t{:?}.{:?}b{:?}s{:?}.json",
                    current_date.year,
                    current_date.month,
                    current_date.day,
                    current_time.hour,
                    current_time.minute,
                    busid.id_number,
                    staid.id_number
                );
                let _ = fs::write(&pathcsv, &serialized_csv);
                let _ = fs::write(&pathjson, &serialized_json);
            }

            pass_in_bus.insert(*busid, newbus);
            println!(
                "  Bus:{:?} Sta:{:?} Pi:{:?} Po:{:?} Pb:{:?}",
                busid.id_number, staid.id_number, pass_to_enter, pass_to_leave, newbus
            );
        }
        current_time.next_minute(Some(&mut current_date));
    }
}
