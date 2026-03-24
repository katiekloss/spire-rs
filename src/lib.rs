pub struct Run {
    pub floor: u32,
    pub relics: Vec<Relic>
}

pub struct EncounterState {
    pub turn: u32,
    pub enemies: Vec<Enemy>,
    pub effects: Vec<Box<dyn Effect>>
}

pub struct Enemy {
    
}

pub trait Effect {

}

pub enum Relic {

}