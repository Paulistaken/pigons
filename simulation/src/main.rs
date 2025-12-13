pub mod simulsimple;
use crate::dane_out::{BusEvent, Date, TDF};

fn main() {
    test_create_output();
    println!("Hello, world!");
    simulsimple::run_symulacja();
}

pub mod dane_symulacji {
    use bevy_ecs::{
        component::Component,
        entity::Entity,
        resource::Resource,
        system::{Query, Res, ResMut},
    };

    use crate::dane_out::BusEvent;
    type Time = crate::dane_out::Time;
    type Date = crate::dane_out::Date;
    type StationIdExport = crate::dane_out::StationId;
    type BusIdExport = crate::dane_out::BusId;

    pub fn symulacja_main() {
        let mut world = bevy_ecs::prelude::World::new();
        let update_time_system = world.register_system(update_time);
        let bus_events_system = world.register_system(buss_station_events);
        world.insert_resource(TimeResource {
            time: Time {
                hour: 12,
                minute: 0,
            },
            data: Date {
                day: 12,
                month: 12,
                year: 2025,
            },
        });

        loop {
            if let Err(e) = world.run_system(bus_events_system) {
                println!("Error: {:?}", e);
            }
            if let Err(e) = world.run_system(update_time_system) {
                println!("Error: {:?}", e);
            }
        }
    }

    //Funckja przesuwa czas do momętu kiedy następny autobus przyjeżdża na dowolną stację
    fn update_time(mut timeres: ResMut<TimeResource>, busses: Query<&BusData>) {
        let curtime = timeres.time;
        let mintime = busses
            .iter()
            .filter_map(|b| b.first_time_to_stop(curtime))
            .min();
        if let Some(mintime) = mintime {
            timeres.time = mintime;
        } else {
            timeres.time = Time { hour: 0, minute: 0 };
            timeres.data.day += 1;
        }
    }

    //Funkcja symuluje wejście lub wyjście z autobusu pasażerów dla określonego momętu w czasie
    fn buss_station_events(
        rtime: Res<TimeResource>,
        mut export: ResMut<EventsResource>,
        mut busses_q: Query<&mut BusData>,
        mut stations_q: Query<&mut Przystanek>,
        passangers_q: Query<&Pasarzer>,
    ) {
        let time = rtime.time;
        let busses_that_stop;
        {
            busses_that_stop = busses_q
                .iter()
                .filter_map(|b| b.runs_at_time(time))
                .collect::<Vec<_>>();
        }
        for event in busses_that_stop {
            let (bid, sid) = (event.bus_id, event.station_id);
            let bussdata = busses_q.get_mut(bid.id);
            let stationdata = stations_q.get_mut(sid.id);
            if bussdata.is_err() || stationdata.is_err() {
                continue;
            }
            let mut bussdata = bussdata.unwrap();
            let mut stationdata = stationdata.unwrap();
            let pasbus = bussdata.get_pass_leave(sid, time, passangers_q);
            let passta = stationdata.get_pass_enter(time, &bussdata, passangers_q);
            let pass_leave = pasbus.iter().filter(|(_, i)| *i).count();
            let pass_enter = passta.iter().filter(|(_, i)| *i).count();
            bussdata.pasangers = pasbus
                .into_iter()
                .filter_map(|(p, i)| if i { None } else { Some(p) })
                .collect::<Vec<_>>();
            stationdata.pasangers = passta
                .into_iter()
                .filter_map(|(p, i)| if i { None } else { Some(p) })
                .collect::<Vec<_>>();
            export.events.push(BusEvent {
                date_of_event: rtime.data,
                time_of_event: rtime.time,
                id_of_the_bus: bussdata.export,
                id_of_the_station: stationdata.export,
                pasangers_coming_out: pass_leave as u32,
                pasangers_coming_in: pass_enter as u32,
                previous_stations: vec![],
                next_stations: vec![],
            });
        }
    }

    #[derive(Resource, Debug, Default)]
    pub struct TimeResource {
        time: Time,
        data: Date,
    }
    #[derive(Resource, Debug, Default)]
    pub struct EventsResource {
        events: Vec<BusEvent>,
    }

    #[derive(Component, Copy, Clone, Debug)]
    struct IdPasarzera {
        id: Entity,
    }
    #[derive(Component, Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
    struct IdPrzystanku {
        id: Entity,
    }
    #[derive(Component, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct IdAutobusu {
        id: Entity,
    }
    #[derive(Component, Clone, Debug)]
    struct DayPlanStep {
        person_id: IdPasarzera,
        station_id: IdPrzystanku,
        from: Time,
        until: Time,
    }
    #[derive(Component, Clone, Debug)]
    struct BusPlanStep {
        bus_id: IdAutobusu,
        station_id: IdPrzystanku,
        time: Time,
    }
    #[derive(Component, Clone, Debug)]
    struct BusData {
        id: IdAutobusu,
        export: BusIdExport,
        plan: Vec<BusPlanStep>,
        pasangers: Vec<IdPasarzera>,
    }
    #[derive(Component)]
    struct Przystanek {
        id: IdPrzystanku,
        export: StationIdExport,
        pasangers: Vec<IdPasarzera>,
    }
    #[derive(Component)]
    struct Pasarzer {
        id: IdPasarzera,
        plan: Vec<DayPlanStep>,
    }
    impl Przystanek {
        fn get_pass_enter(
            &self,
            time: Time,
            bussdata: &BusData,
            passq: Query<&Pasarzer>,
        ) -> Vec<(IdPasarzera, bool)> {
            self.pasangers
                .iter()
                .map(|p| {
                    (
                        *p,
                        passq
                            .get(p.id)
                            .is_ok_and(|p| p.enters_the_bus(self.id, time, bussdata)),
                    )
                })
                .collect::<Vec<_>>()
        }
    }
    impl BusData {
        //Gets the first step moment when bus stops ((>=)) after timebg
        fn first_time_to_stop(&self, timebg: Time) -> Option<Time> {
            self.plan.iter().find(|p| p.time > timebg).map(|p| p.time)
        }
        fn get_pass_leave(
            &self,
            sid: IdPrzystanku,
            time: Time,
            passq: Query<&Pasarzer>,
        ) -> Vec<(IdPasarzera, bool)> {
            self.pasangers
                .iter()
                .map(|p| {
                    (
                        *p,
                        passq.get(p.id).is_ok_and(|p| p.exits_at_station(time, sid)),
                    )
                })
                .collect::<Vec<_>>()
        }
        fn stops_at_station(&self, sid: IdPrzystanku, time_bg: Time) -> Option<Time> {
            self.plan
                .iter()
                .filter(|p| p.time >= time_bg)
                .find(|p| p.station_id == sid)
                .map(|p| p.time)
        }
        fn runs_at_time(&self, time: Time) -> Option<BusPlanStep> {
            let pln = self.plan.clone();
            pln.into_iter().find(|b| b.time == time)
        }
    }
    impl Pasarzer {
        fn enters_the_bus(&self, csid: IdPrzystanku, time: Time, bussdata: &BusData) -> bool {
            if self
                .plan
                .iter()
                .any(|p| p.station_id == csid && p.until <= time && p.from >= time)
            {
                return false;
            }
            self.plan.iter().filter(|p| p.until > time).any(|p| {
                let ntime = bussdata.stops_at_station(p.station_id, time);
                if let Some(ntime) = ntime {
                    ntime <= p.until
                } else {
                    false
                }
            })
        }
        fn exits_at_station(&self, time: Time, sid: IdPrzystanku) -> bool {
            let pln = self.plan.clone();
            pln.iter().any(|p| p.until <= time && p.station_id == sid)
        }
    }
}

pub mod dane_out {

    #[derive(
        Default,
        Debug,
        Clone,
        serde_derive::Serialize,
        serde_derive::Deserialize,
        bevy_ecs::component::Component,
    )]
    pub struct PlannedStation {
        pub id_of_the_bus: BusId,
        pub id_of_the_station: StationId,
        pub time_of_event: Time,
    }
    #[derive(
        Default,
        Debug,
        Clone,
        serde_derive::Serialize,
        serde_derive::Deserialize,
        bevy_ecs::component::Component,
    )]
    pub struct BusEvent {
        pub date_of_event: Date,
        pub time_of_event: Time,
        pub id_of_the_bus: BusId,
        pub id_of_the_station: StationId,
        pub pasangers_coming_in: u32,
        pub pasangers_coming_out: u32,
        pub previous_stations: Vec<PlannedStation>,
        pub next_stations: Vec<PlannedStation>,
    }

    pub trait TDF {
        fn etd(&self) -> String;
        fn export_tdf(&self) -> String;
    }
    impl TDF for Date {
        fn etd(&self) -> String {
            " [DATE:█YEAR,MONTH,DAY█] ".to_string()
        }
        fn export_tdf(&self) -> String {
            format!("{:?},{:?},{:?}", self.year, self.month, self.day)
        }
    }
    impl TDF for Time {
        fn etd(&self) -> String {
            " [TIME:█HOUR,MINUTE█] ".to_string()
        }
        fn export_tdf(&self) -> String {
            format!("{:?},{:?}", self.hour, self.minute)
        }
    }
    impl BusEvent {
        pub fn etd_plan(&self) -> String {
            format!(
                " [BUSPLAN:█BUSID, [REPEATING:█STATIONID,{},█] █] ",
                self.time_of_event.etd()
            )
        }
        pub fn export_tdf_plan(&self) -> String {
            let mut res = "".to_string();
            res += &format!("{:?},", self.id_of_the_bus.id_number);
            for step in self.previous_stations.iter() {
                res += &format!(
                    "{:?},{},",
                    step.id_of_the_station.id_number,
                    step.time_of_event.export_tdf()
                );
            }
            res += &format!(
                "{:?},{},",
                self.id_of_the_station.id_number,
                self.time_of_event.export_tdf()
            );
            for step in self.next_stations.iter() {
                res += &format!(
                    "{:?},{},",
                    step.id_of_the_station.id_number,
                    step.time_of_event.export_tdf()
                );
            }
            res
        }
    }
    impl TDF for BusEvent {
        fn etd(&self) -> String {
            format!(
                " [BusEVENT:█{},{},BUSNUM,STATIONNUM,PASSIN,PASSOUT█] ",
                self.time_of_event.etd(),
                self.date_of_event.etd()
            )
        }
        fn export_tdf(&self) -> String {
            format!(
                "{},{},{:?},{:?},{:?},{:?}",
                self.time_of_event.export_tdf(),
                self.date_of_event.export_tdf(),
                self.id_of_the_bus.id_number,
                self.id_of_the_station.id_number,
                self.pasangers_coming_in,
                self.pasangers_coming_out,
            )
        }
    }

    #[derive(
        Default,
        Debug,
        Clone,
        serde_derive::Serialize,
        serde_derive::Deserialize,
        bevy_ecs::component::Component,
    )]
    struct BusPlan {
        pub id_autobusu: BusId,
        pub plan: Vec<PlannedStation>,
    }
    #[derive(
        Default,
        Debug,
        Clone,
        Copy,
        serde_derive::Serialize,
        serde_derive::Deserialize,
        bevy_ecs::component::Component,
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
        bevy_ecs::component::Component,
    )]
    pub struct BusId {
        pub id_number: u32,
    }
    const MONTHS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    #[derive(
        Debug,
        Clone,
        Copy,
        serde_derive::Serialize,
        serde_derive::Deserialize,
        bevy_ecs::component::Component,
    )]
    pub struct Date {
        pub year: u32,
        pub month: u32,
        pub day: u32,
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
        bevy_ecs::component::Component,
        Eq,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct Time {
        pub hour: u32,
        pub minute: u32,
    }
    impl Time {
        pub fn next_minute(&mut self, date: Option<&mut Date>) {
            self.minute += 1;
            if self.minute >= 60 {
                self.hour += 1;
            }
            if self.hour >= 12 {
                if let Some(date) = date {
                    date.next_day();
                }
            }
        }
    }
}
fn test_create_output() {
    let mut exampledata;
    {
        let bus_id = dane_out::BusId { id_number: 501 };
        exampledata = BusEvent {
            date_of_event: Date {
                year: 2025,
                month: 12,
                day: 9,
            },
            time_of_event: dane_out::Time {
                hour: 10,
                minute: 27,
            },
            id_of_the_bus: bus_id,
            id_of_the_station: dane_out::StationId { id_number: 17 },
            pasangers_coming_in: 25,
            pasangers_coming_out: 17,
            next_stations: vec![
                dane_out::PlannedStation {
                    id_of_the_bus: dane_out::BusId { id_number: 501 },
                    id_of_the_station: dane_out::StationId { id_number: 19 },
                    time_of_event: dane_out::Time {
                        hour: 10,
                        minute: 32,
                    },
                },
                dane_out::PlannedStation {
                    id_of_the_bus: dane_out::BusId { id_number: 501 },
                    id_of_the_station: dane_out::StationId { id_number: 23 },
                    time_of_event: dane_out::Time {
                        hour: 10,
                        minute: 54,
                    },
                },
                dane_out::PlannedStation {
                    id_of_the_bus: dane_out::BusId { id_number: 501 },
                    id_of_the_station: dane_out::StationId { id_number: 17 },
                    time_of_event: dane_out::Time {
                        hour: 11,
                        minute: 12,
                    },
                },
                dane_out::PlannedStation {
                    id_of_the_bus: dane_out::BusId { id_number: 501 },
                    id_of_the_station: dane_out::StationId { id_number: 35 },
                    time_of_event: dane_out::Time {
                        hour: 11,
                        minute: 48,
                    },
                },
            ],
            previous_stations: vec![
                dane_out::PlannedStation {
                    id_of_the_bus: dane_out::BusId { id_number: 501 },
                    id_of_the_station: dane_out::StationId { id_number: 16 },
                    time_of_event: dane_out::Time {
                        hour: 10,
                        minute: 5,
                    },
                },
                dane_out::PlannedStation {
                    id_of_the_bus: dane_out::BusId { id_number: 501 },
                    id_of_the_station: dane_out::StationId { id_number: 12 },
                    time_of_event: dane_out::Time {
                        hour: 9,
                        minute: 55,
                    },
                },
                dane_out::PlannedStation {
                    id_of_the_bus: dane_out::BusId { id_number: 501 },
                    id_of_the_station: dane_out::StationId { id_number: 43 },
                    time_of_event: dane_out::Time {
                        hour: 9,
                        minute: 27,
                    },
                },
                dane_out::PlannedStation {
                    id_of_the_bus: dane_out::BusId { id_number: 501 },
                    id_of_the_station: dane_out::StationId { id_number: 5 },
                    time_of_event: dane_out::Time { hour: 9, minute: 1 },
                },
                dane_out::PlannedStation {
                    id_of_the_bus: dane_out::BusId { id_number: 501 },
                    id_of_the_station: dane_out::StationId { id_number: 22 },
                    time_of_event: dane_out::Time {
                        hour: 8,
                        minute: 34,
                    },
                },
            ],
        };
    }
    {
        let serializedt = exampledata.etd();
        let serialized = exampledata.export_tdf();
        let _ = std::fs::write("exampleout1tmp.txt", &serializedt);
        let _ = std::fs::write("exampleout1.csv", &serialized);
    }
    {
        let serialized = serde_json::to_string(&exampledata).unwrap();
        let _ = std::fs::write("exampleout1.json", &serialized);
    }
    {
        let serialized = serde_toon::to_string(&exampledata).unwrap();
        let _ = std::fs::write("exampleout1.toon", &serialized);
    }
    exampledata
        .previous_stations
        .push(dane_out::PlannedStation {
            id_of_the_bus: exampledata.id_of_the_bus,
            id_of_the_station: exampledata.id_of_the_station,
            time_of_event: exampledata.time_of_event,
        });
    exampledata.id_of_the_station = exampledata.next_stations[0].id_of_the_station;
    exampledata.time_of_event = exampledata.next_stations[0].time_of_event;
    exampledata.pasangers_coming_out = 25;
    exampledata.pasangers_coming_in = 8;
    exampledata.next_stations = exampledata
        .next_stations
        .into_iter()
        .skip(1)
        .collect::<Vec<_>>();
    {
        let serializedt = exampledata.etd();
        let serialized = exampledata.export_tdf();
        let _ = std::fs::write("exampleout2tmp.txt", &serializedt);
        let _ = std::fs::write("exampleout2.csv", &serialized);
    }
    {
        let serialized = serde_json::to_string(&exampledata).unwrap();
        let _ = std::fs::write("exampleout2.json", &serialized);
    }
    {
        let serialized = serde_toon::to_string(&exampledata).unwrap();
        let _ = std::fs::write("exampleout2.toon", &serialized);
    }
    {
        let serializedt = exampledata.etd_plan();
        let serialized = exampledata.export_tdf_plan();
        let _ = std::fs::write("exampleoutplantmp.txt", &serializedt);
        let _ = std::fs::write("exampleoutplan.csv", &serialized);
    }
}
