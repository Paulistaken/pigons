type Time = crate::dane_out::Time;
type Date = crate::dane_out::Date;
type BusID = crate::dane_out::BusId;
type StaId = crate::dane_out::StationId;

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
    pub level: f32,
}
#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SimuStation {
    pub staid: StaId,
    pub entering_attractivness: Option<Vec<Attractivness>>,
    pub leaving_attractivness: Option<Vec<Attractivness>>,
}
#[derive(Default, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SimulationInput {
    pub start_time: Time,
    pub start_date: Date,
    pub end_date: Date,
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
    pub attractivness_e: [Option<f32>; 24],
    pub attractivness_l: [Option<f32>; 24],
}
impl From<SimuStation> for StationData {
    fn from(value: SimuStation) -> Self {
        let mut att_e = [None; 24];
        let mut att_l = [None; 24];
        if let Some(attr) = value.entering_attractivness {
            for pt in attr {
                for hr in pt.start_hour..=pt.end_hour {
                    att_e[hr as usize] = Some(pt.level);
                }
            }
        }
        if let Some(attr) = value.leaving_attractivness {
            for pt in attr {
                for hr in pt.start_hour..=pt.end_hour {
                    att_l[hr as usize] = Some(pt.level);
                }
            }
        }
        Self {
            _id: value.staid,
            attractivness_e: att_e,
            attractivness_l: att_l,
        }
    }
}
