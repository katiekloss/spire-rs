use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use crate::{Damageable, Effect, Effectable, Target, Team, cards::library::Card};

static ENEMY_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1)); // the player is 0
static MONSTERS: LazyLock<HashMap<Monsters, MonsterData>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Monsters::FuzzyWurmCrawler, MonsterData {
        health: 55,
        moves: vec![
            Moves::Attack(4),
            Moves::Attack(4),
            Moves::Buff(Effect::Strength(7))]
    });
    m.insert(Monsters::SmallLeafSlime, MonsterData {
        health: 11,
        moves: vec![
            Moves::Attack(3),
            Moves::StatusCard(Card::Slimed)
        ]
    });
    m.insert(Monsters::MediumLeafSlime, MonsterData {
        health: 32,
        moves: vec![
            Moves::Attack(8),
            Moves::StatusCard(Card::Slimed)
        ]
    });
    m.insert(Monsters::SmallTwigSlime, MonsterData {
        health: 7,
        moves: vec![
            Moves::Attack(4)
        ]
    });
    m.insert(Monsters::MediumTwigSlime, MonsterData {
        health: 26,
        moves: vec![
            Moves::Attack(11),
            Moves::StatusCard(Card::Slimed)
        ]
    });
    m
});

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Monsters {
    FuzzyWurmCrawler,
    SmallLeafSlime,
    MediumLeafSlime,
    SmallTwigSlime,
    MediumTwigSlime
}

#[derive(Clone)]
pub enum Moves {
    Attack(u32),
    Buff(Effect),
    Debuff(Effect),
    StatusCard(Card)
}

pub struct MonsterData {
    health: u32,
    moves: Vec<Moves>
}

pub struct Enemy {
    pub id: u32,
    pub monster: Monsters,
    pub health: u32,
    pub block: u32,
    pub effects: Vec<Effect>,
    pub move_idx: usize,
    pub moves: Vec<Moves>
}

impl Enemy {
    pub fn new(monster: Monsters) -> Self {
        Self {
            id: { let mut i = ENEMY_ID.lock().unwrap(); *i += 1; *i },
            health: MONSTERS[&monster].health,
            moves: MONSTERS[&monster].moves.clone(),
            monster,
            block: 0,
            effects: vec![],
            move_idx: 0,
        }
    }
}

impl PartialEq for Enemy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Damageable for Enemy {
    fn get_block(&self) -> u32 {
        self.block
    }

    fn get_health(&self) -> u32 {
        self.health
    }

    fn set_block(&mut self, block: u32) {
        self.block = block;
    }

    fn set_health(&mut self, health: u32) {
        self.health = health;
    }
}

impl Effectable for Enemy {
    fn get_effects(&self) -> &Vec<Effect> {
        &self.effects
    }
}

impl Target for Enemy {
    fn get_team(&self) -> crate::Team {
        Team::Enemy
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}
