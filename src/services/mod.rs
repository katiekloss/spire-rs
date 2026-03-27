use crate::{Effect, Run, cards::CardInstance, monsters::Monster};


pub struct EncounterManager {
    pub energy: u32,
    pub block: u32,
    pub hand: Vec<CardInstance>,

    pub turn: u32,
    pub health: u32,
    pub enemies: Vec<Box<dyn Monster>>,
    pub effects: Vec<Box<dyn Effect>>
}

impl<'a> EncounterManager {
    pub fn new(run: &Run) -> Self {
        Self {
            turn: 1,
            health: run.health,
            enemies: vec![],
            effects: vec![],
            energy: 3,
            block: 0,
            hand: vec![]
    }}
    
    pub fn begin_turn(&mut self) {
    }

    pub fn end_turn(&mut self) {
        self.turn += 1;
    }

    pub fn play(&self, _card: &CardInstance) {

    }
}