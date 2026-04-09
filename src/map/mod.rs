use rand::RngExt;

use crate::{monsters::Monsters, relics::Relics};

#[derive(Clone)]
pub struct MapRoom {
    pub t: RoomType,
    pub up_nodes: Vec<MapRoom>
}

#[derive(Clone)]
pub enum RoomType {
    Ancient(Ancients),
    Encounter(Vec<Monsters>, u32),
    Treasure(Relics, u32),
    Elite(Vec<Monsters>, u32),
    Rest
}

#[derive(Clone)]
pub enum Ancients {
    Neow
}

pub struct MapGenerator {
}

impl MapGenerator {
    pub fn generate() -> MapRoom {
        let mut rng = rand::rng();

        MapRoom {
            t: RoomType::Ancient(Ancients::Neow),
            up_nodes: vec![
                MapRoom {
                    t: RoomType::Encounter(vec![Monsters::FuzzyWurmCrawler], rng.random_range(10..20)),
                    up_nodes: vec![
                        MapRoom {
                            t: RoomType::Encounter(vec![Monsters::SmallLeafSlime, Monsters::MediumLeafSlime, Monsters::SmallTwigSlime, Monsters::MediumTwigSlime], rng.random_range(10..20)),
                            up_nodes: vec![
                                MapRoom {
                                    t: RoomType::Encounter(vec![Monsters::MediumLeafSlime, Monsters::FuzzyWurmCrawler], rng.random_range(10..20)),
                                    up_nodes: vec![
                                        MapRoom {
                                            t: RoomType::Treasure(Relics::RingOfTheSnake, rng.random_range(42..52)),
                                            up_nodes: vec![
                                                MapRoom {
                                                    t: RoomType::Elite(vec![Monsters::Byrdonis], rng.random_range(35..45)),
                                                    up_nodes: vec![]
                                                }]
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                },
            ]
        }
    }
}