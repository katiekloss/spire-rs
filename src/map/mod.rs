use crate::{monsters::Monsters, relics::Relics};

#[derive(Clone)]
pub struct MapRoom {
    pub t: RoomType,
    pub up_nodes: Vec<MapRoom>
}

#[derive(Clone)]
pub enum RoomType {
    Ancient(Ancients),
    Encounter(Vec<Monsters>),
    Treasure(Relics, u32),
    Elite(Vec<Monsters>),
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
        MapRoom {
            t: RoomType::Ancient(Ancients::Neow),
            up_nodes: vec![
                MapRoom {
                    t: RoomType::Encounter(vec![Monsters::FuzzyWurmCrawler]),
                    up_nodes: vec![
                        MapRoom {
                            t: RoomType::Encounter(vec![Monsters::SmallLeafSlime, Monsters::MediumLeafSlime, Monsters::SmallTwigSlime, Monsters::MediumTwigSlime]),
                            up_nodes: vec![
                                MapRoom {
                                    t: RoomType::Encounter(vec![Monsters::MediumLeafSlime, Monsters::FuzzyWurmCrawler]),
                                    up_nodes: vec![
                                        MapRoom {
                                            t: RoomType::Treasure(Relics::RingOfTheSnake, 15),
                                            up_nodes: vec![
                                                MapRoom {
                                                    t: RoomType::Elite(vec![Monsters::Byrdonis]),
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