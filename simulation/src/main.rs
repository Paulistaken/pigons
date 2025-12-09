use crate::dane_out::{BusEvent, Date};

fn main() {
    test_create_output();
    println!("Hello, world!");
}

pub mod dane_symulacji {
    use bevy_ecs::{component::Component, entity::Entity};
    type Time = crate::dane_out::Time;
    type Date = crate::dane_out::Date;
    type StationIdExport = crate::dane_out::StationId;
    type BusIdExport = crate::dane_out::BusId;

    #[derive(Component, Copy, Clone, Debug)]
    struct IdPasarzera {
        id: Entity,
    }
    #[derive(Component, Copy, Clone, Debug)]
    struct IdPrzystanku {
        id: Entity,
    }
    #[derive(Component, Copy, Clone, Debug)]
    struct IdAutobusu {
        id: Entity,
    }
    #[derive(Component, Clone, Debug)]
    struct DayPlanStep {
        person_id : IdPasarzera,
        station_id : IdPrzystanku,
        from : Time,
        until : Time,
    }
    #[derive(Component, Clone, Debug)]
    struct BusPlanStep {
        bus_id : IdAutobusu,
        station_id : IdPrzystanku,
        time : Time,
    }
    #[derive(Component, Clone, Debug)]
    struct BusData {
        id: IdAutobusu,
        export : BusIdExport,
        plan: Vec<BusPlanStep>,
        pasangers: Vec<IdPasarzera>,
    }
    #[derive(Component)]
    struct Przystanek {
        id: IdPrzystanku,
        export : StationIdExport,
        pasangers: Vec<IdPasarzera>,
    }
    #[derive(Component)]
    struct Pasarzer {
        id: IdPasarzera,
        plan: Vec<DayPlanStep>,
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
        serde_derive::Serialize,
        serde_derive::Deserialize,
        bevy_ecs::component::Component,
    )]
    pub struct BusId {
        pub id_number: u32,
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
    pub struct Date {
        pub year: u32,
        pub month: u32,
        pub day: u32,
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
    pub struct Time {
        pub hour: u32,
        pub minute: u32,
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
        let serialized = serde_json::to_string(&exampledata).unwrap();
        let _ = std::fs::write("exampleout1.json", &serialized);
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
        let serialized = serde_json::to_string(&exampledata).unwrap();
        let _ = std::fs::write("exampleout2.json", &serialized);
    }
}
