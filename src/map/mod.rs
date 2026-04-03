pub struct MapRoom {
    pub t: RoomType,
    pub up_nodes: Vec<MapRoom>
}

pub enum RoomType {
    Ancient(Ancients),
    Encounter
}

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
                    t: RoomType::Encounter,
                    up_nodes: vec![]
                },
                MapRoom {
                    t: RoomType::Encounter,
                    up_nodes: vec![]
                }
        ] }
    }
}