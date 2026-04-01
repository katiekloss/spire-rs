use std::sync::{LazyLock, Mutex};

use crate::Effect;

static ENEMY_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

pub struct Enemy {
    pub id: u32,
    pub monster: Monsters,
    pub health: u32,
    pub block: u32,
    pub effects: Vec<Box<dyn Effect>>
}

impl PartialEq for Enemy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub enum Monsters {
    FuzzyWurmCrawler
}

impl Enemy {
    pub fn new(monster: Monsters) -> Self {
        Self {
            id: { let mut i = ENEMY_ID.lock().unwrap(); *i += 1; *i },
            monster,
            health: 300,
            block: 0,
            effects: vec![]
        }
    }
}