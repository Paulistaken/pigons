use crate::dane_out::TDF;
use crate::dane_out::{BusEvent, BusId as BusID, Date, StationId as StaId, Time};
use crate::simplestucts::SimulationInput;
use std::collections::HashMap;
use std::fs;

const LOGS_PATH: &str = "simresults/pre";
const CSV_LOGS_PATH: &str = "simresults/pre/csv";
const JSON_LOGS_PATH: &str = "simresults/pre/json";
const BUS_LOGS_PATH: &str = "simresults/pre/bus_logs";

pub fn setup_paths() {
    {
        let _ = std::fs::remove_dir_all(LOGS_PATH);
        let _ = std::fs::create_dir_all(CSV_LOGS_PATH);
        let _ = std::fs::create_dir_all(JSON_LOGS_PATH);
        let _ = std::fs::create_dir_all(BUS_LOGS_PATH);
    }
}
pub fn get_export_data(
    current_time: &Time,
    current_date: &Date,
    id_bus: &BusID,
    id_station: &StaId,
    passangers_entering_amount: u32,
    passangers_leaving_amount: u32,
    passangers_in_bus_debug: u32,
) -> ((String, BusEvent), (BusEvent, u32)) {
    let export_data = BusEvent {
        date_of_event: *current_date,
        time_of_event: *current_time,
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
    (
        (pathname, export_data.clone()),
        (export_data, passangers_in_bus_debug),
    )
}
pub fn export_logs(export_logs: &[(String, BusEvent)]) {
    println!("Exporting {} event logs", export_logs.len());
    for (export_path, export_data) in export_logs {
        let serialized_csv = export_data.export_tdf();
        let serialized_csv = serialized_csv.0 + "\n" + &serialized_csv.1;
        let serialized_json = serde_json::to_string(&export_data).unwrap_or("".to_string());
        let pathcsv = CSV_LOGS_PATH.to_string() + "./BusEVENT" + export_path + ".csv";
        let pathjson = JSON_LOGS_PATH.to_string() + "/BusEVENT" + export_path + ".json";
        let _ = fs::write(&pathcsv, &serialized_csv);
        let _ = fs::write(&pathjson, &serialized_json);
    }
}
pub fn export_bus_line_logs(
    simulinput: &SimulationInput,
    bus_line_logs: &HashMap<BusID, Vec<(BusEvent, u32)>>,
) {
    for (bus_id, export_data) in bus_line_logs {
        println!(
            "Exporting {} event logs for bus {}",
            export_data.len(),
            bus_id.id_number
        );
        let path = format!("{}/bus{}.txt", BUS_LOGS_PATH, bus_id.id_number);
        let path2 = format!("{}/bus{}.csv", BUS_LOGS_PATH, bus_id.id_number);
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
}
