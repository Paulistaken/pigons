pub mod bus_ai;
pub mod dane_out;
pub mod export_logs_mod;
pub mod passanger_ai;
pub mod setup_simulation;
pub mod simplestucts;
pub mod simulation_tick;
pub mod simulsimple;

fn main() {
    let simulation_input_data_raw =
        std::fs::read_to_string("siminput.json").expect("Error, no siminput.json file");

    let simulatin_input_data: simplestucts::SimulationInput =
        serde_json::from_str(&simulation_input_data_raw).unwrap();

    let mut simulation_state = simulsimple::SimulationState::new(simulatin_input_data);

    simulation_state.run_simulation();
}
