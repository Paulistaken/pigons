

pub type Time = crate::dane_out::Time;
pub type Date = crate::dane_out::Date;
pub type BusID = crate::dane_out::BusId;
pub type StaId = crate::dane_out::StationId;
#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SimuBusPlan {
    pub busid: BusID,
    pub points: Vec<SimuStopPoint>,
}
#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SimuStopPoint {
    pub time: Time,
    pub staid: StaId,
}
#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct Attractivness {
    pub start_hour: u32,
    pub end_hour: u32,
    pub att: f32,
}
#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SimuStation {
    pub staid: StaId,
    pub atract: Option<Vec<Attractivness>>,
}
#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SimulationInput {
    pub start_time: Time,
    pub start_date: Date,
    pub stations: Vec<SimuStation>,
    pub bus_plans: Vec<SimuBusPlan>,
}
#[derive(Debug, Clone)]
pub struct Passanger {
    pub station_to_leave: StaId,
}
impl Passanger {
    pub fn new(id: StaId) -> Self {
        Self {
            station_to_leave: id,
        }
    }
}
#[derive(Debug)]
pub struct StationData {
    pub _id: StaId,
    pub attractivness: [Option<f32>; 24],
}
impl From<SimuStation> for StationData {
    fn from(value: SimuStation) -> Self {
        let mut att = [None; 24];
        if let Some(attr) = value.atract {
            for pt in attr {
                for hr in pt.start_hour..pt.end_hour {
                    att[hr as usize] = Some(pt.att);
                }
            }
        }
        Self {
            _id: value.staid,
            attractivness: att,
        }
    }
}
