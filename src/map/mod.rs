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

        let rooms = vec![
            RoomType::Ancient(Ancients::Neow),
            RoomType::Encounter(vec![Monsters::FuzzyWurmCrawler], rng.random_range(10..20)),
            RoomType::Encounter(vec![Monsters::SmallLeafSlime, Monsters::MediumLeafSlime, Monsters::SmallTwigSlime, Monsters::MediumTwigSlime], rng.random_range(10..20)),
            RoomType::Encounter(vec![Monsters::MediumLeafSlime, Monsters::FuzzyWurmCrawler], rng.random_range(10..20)),
            RoomType::Encounter(vec![Monsters::MediumLeafSlime, Monsters::FuzzyWurmCrawler], rng.random_range(10..20)), // new monster here
            RoomType::Rest,
            RoomType::Elite(vec![Monsters::Byrdonis], rng.random_range(35..45)),
            RoomType::Encounter(vec![Monsters::SmallLeafSlime, Monsters::MediumLeafSlime, Monsters::SmallTwigSlime, Monsters::MediumTwigSlime], rng.random_range(10..20)),
            RoomType::Rest,
            RoomType::Treasure(Relics::RingOfTheSnake, rng.random_range(42..52)),
            RoomType::Encounter(vec![Monsters::MediumLeafSlime, Monsters::FuzzyWurmCrawler], rng.random_range(10..20)),
            RoomType::Rest,
            RoomType::Encounter(vec![Monsters::SmallLeafSlime, Monsters::MediumLeafSlime, Monsters::SmallTwigSlime, Monsters::MediumTwigSlime], rng.random_range(10..20)),
            RoomType::Elite(vec![Monsters::Byrdonis], rng.random_range(35..45)),
            RoomType::Encounter(vec![Monsters::MediumLeafSlime, Monsters::FuzzyWurmCrawler], rng.random_range(10..20)),
            RoomType::Rest,
            // boss
        ];

        let mut current_room = MapRoom {
            t: rooms[rooms.len() - 1].clone(),
            up_nodes: vec![]
        };

        for i in (0 .. rooms.len() - 1).rev() {
            current_room = MapRoom {
                t: rooms[i].clone(),
                up_nodes: vec![current_room]
            };
        }

        current_room
    }
}