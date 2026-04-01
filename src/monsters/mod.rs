use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use crate::Effect;

static ENEMY_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
static MONSTERS: LazyLock<HashMap<Monsters, MonsterData>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Monsters::FuzzyWurmCrawler, MonsterData { health: 55, moves: vec![Moves::Attack(4), Moves::Attack(4), Moves::Apply(Effect::Strength(7))] });
    m
});

pub struct MonsterData {
    health: u32,
    moves: Vec<Moves>
}

pub struct Enemy {
    pub id: u32,
    pub monster: Monsters,
    pub health: u32,
    pub block: u32,
    pub effects: Vec<Effect>
}

impl PartialEq for Enemy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum Monsters {
    FuzzyWurmCrawler
}

pub enum Moves {
    Attack(u32),
    Apply(Effect)
}

impl Enemy {
    pub fn new(monster: Monsters) -> Self {
        Self {
            id: { let mut i = ENEMY_ID.lock().unwrap(); *i += 1; *i },
            health: MONSTERS[&monster].health,
            monster,
            block: 0,
            effects: vec![]
        }
    }
}