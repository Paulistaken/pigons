
#[derive(Default, Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PlannedStation {
    pub id_of_the_bus: BusId,
    pub id_of_the_station: StationId,
    pub time_of_event: Time,
}
#[derive(Default, Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BusEvent {
    pub date_of_event: Date,
    pub time_of_event: Time,
    pub id_of_the_bus: BusId,
    pub id_of_the_station: StationId,
    pub pasangers_coming_in: u32,
    pub pasangers_coming_out: u32,
}
#[derive(Default, Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BusSchedule {
    pub id_of_the_bus: BusId,
    pub stops: Vec<(StationId, Time)>,
}

pub trait TDF {
    fn export_tdf(&self) -> (String, String);
}
impl TDF for BusSchedule {
    fn export_tdf(&self) -> (String, String) {
        let mut uprow = "BusId".to_string();
        let mut downrow = format!("{}", self.id_of_the_bus.id_number);
        for (i, (station, time)) in self.stops.iter().enumerate() {
            let time_tdf = time.export_tdf();
            uprow += &format!(",stopNumber,station,{}", time_tdf.0);
            downrow += &format!(",{},{},{}", i, station.id_number, time_tdf.1);
        }
        (uprow, downrow)
    }
}
impl TDF for Date {
    fn export_tdf(&self) -> (String, String) {
        (
            "year,month,day".to_string(),
            format!("{:?},{:?},{:?}", self.year, self.month, self.day),
        )
    }
}
impl TDF for Time {
    fn export_tdf(&self) -> (String, String) {
        (
            "hour,minute".to_string(),
            format!("{:?},{:?}", self.hour, self.minute),
        )
    }
}
impl TDF for BusEvent {
    fn export_tdf(&self) -> (String, String) {
        let timeexp = self.time_of_event.export_tdf();
        let dateexp = self.date_of_event.export_tdf();
        (
            format!(
                "{},{},bus id,station id,passangers going in,passangers going out",
                timeexp.0, dateexp.0
            ),
            format!(
                "{},{},{:?},{:?},{:?},{:?}",
                timeexp.1,
                dateexp.1,
                self.id_of_the_bus.id_number,
                self.id_of_the_station.id_number,
                self.pasangers_coming_in,
                self.pasangers_coming_out,
            ),
        )
    }
}

#[derive(
    Default,
    Debug,
    Clone,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    // bevy_ecs::component::Component,
)]
struct BusPlan {
    pub id_autobusu: BusId,
    pub plan: Vec<PlannedStation>,
}
#[derive(
    Default,
    Debug,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Copy,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    // bevy_ecs::component::Component,
)]
pub struct StationId {
    pub id_number: u32,
}
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    // bevy_ecs::component::Component,
)]
pub struct BusId {
    pub id_number: u32,
}
const MONTHS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    // bevy_ecs::component::Component,
)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}
impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.year.cmp(&other.year) {
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            _ => {}
        }
        match self.month.cmp(&other.month) {
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            _ => {}
        }
        match self.day.cmp(&other.day) {
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        }
    }
}
impl Default for Date {
    fn default() -> Self {
        Self {
            year: 2025,
            month: 12,
            day: 12,
        }
    }
}
impl Date {
    pub fn next_day(&mut self) {
        self.day += 1;
        if self.day > MONTHS[(self.month - 1) as usize] {
            self.day = 1;
            self.month += 1;
        }
        if self.month > 12 {
            self.month = 1;
            self.year += 1;
        }
    }
}
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    // bevy_ecs::component::Component,
    Eq,
    PartialEq,
)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
}
impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hour > other.hour {
            std::cmp::Ordering::Greater
        } else if self.hour == other.hour {
            if self.minute > other.minute {
                std::cmp::Ordering::Greater
            } else if self.minute == other.minute {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            }
        } else {
            std::cmp::Ordering::Less
        }
    }
}
impl Time {
    pub fn next_minute(&mut self, date: Option<&mut Date>) {
        self.minute += 1;
        if self.minute >= 60 {
            self.hour += 1;
            self.minute = 0;
        }
        if self.hour >= 24 {
            self.hour = 0;
            if let Some(date) = date {
                date.next_day();
            }
        }
    }
}
