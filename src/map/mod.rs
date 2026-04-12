
use rand::{RngExt, distr::Uniform, rngs::ThreadRng};

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
            Self::weak(&mut rng),
            Self::weak(&mut rng),
            Self::weak(&mut rng), // new monster here
            RoomType::Rest,
            Self::elite(&mut rng),
            Self::normal(&mut rng),
            RoomType::Rest,
            RoomType::Treasure(Relics::RingOfTheSnake, rng.random_range(42..52)),
            Self::normal(&mut rng),
            RoomType::Rest,
            Self::normal(&mut rng),
            Self::elite(&mut rng),
            Self::normal(&mut rng),
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

    fn weak(rng: &mut ThreadRng) -> RoomType {
        match rng.sample(Uniform::new_inclusive(1, 3).unwrap()) {
            1 => RoomType::Encounter(vec![Monsters::FuzzyWurmCrawler], rng.random_range(10..=20)),
            2 => RoomType::Encounter(vec![Monsters::ShrinkerBeetle], rng.random_range(10..=20)),
            3 => RoomType::Encounter(vec![Monsters::SmallLeafSlime, Monsters::MediumLeafSlime, Monsters::SmallTwigSlime], rng.random_range(10..=20)),
            _ => unreachable!()
        }
    }

    fn normal(rng: &mut ThreadRng) -> RoomType {
        match rng.sample(Uniform::new_inclusive(1, 3).unwrap()) {
            1 | 2 => RoomType::Encounter(vec![Monsters::SmallLeafSlime, Monsters::MediumLeafSlime, Monsters::SmallTwigSlime, Monsters::MediumTwigSlime], rng.random_range(10..=20)),
            // cubex construct
            3 => RoomType::Encounter(vec![Monsters::ShrinkerBeetle, Monsters::FuzzyWurmCrawler], rng.random_range(10..=20)),
            _ => unreachable!()
        }
    }

    fn elite(rng: &mut ThreadRng) -> RoomType {
        if rng.random_bool(0.5) {
            RoomType::Elite(vec![Monsters::Byrdonis], rng.random_range(35..=45))
        } else {
            RoomType::Elite(vec![Monsters::BygoneEffigy], rng.random_range(35..=45))
        }
    }
}