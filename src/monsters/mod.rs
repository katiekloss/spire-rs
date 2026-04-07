use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use crate::{Damageable, Effect, Effectable, Target, Team, cards::library::Card};

static ENEMY_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1)); // the player is 0
static MONSTERS: LazyLock<HashMap<Monsters, MonsterData>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Monsters::FuzzyWurmCrawler, MonsterData {
        health: 55,
        moves: vec![
            vec![Moves::Attack(4)],
            vec![Moves::Attack(4)],
            vec![Moves::Buff(Effect::Strength(7))]],
        starting_effects: vec![]
    });
    m.insert(Monsters::SmallLeafSlime, MonsterData {
        health: 11,
        moves: vec![
            vec![Moves::Attack(3)],
            vec![Moves::StatusCard(Card::Slimed)]
        ],
        starting_effects: vec![]
    });
    m.insert(Monsters::MediumLeafSlime, MonsterData {
        health: 32,
        moves: vec![
            vec![Moves::Attack(8)],
            vec![Moves::StatusCard(Card::Slimed)]
        ],
        starting_effects: vec![]
    });
    m.insert(Monsters::SmallTwigSlime, MonsterData {
        health: 7,
        moves: vec![
            vec![Moves::Attack(4)]
        ],
        starting_effects: vec![]
    });
    m.insert(Monsters::MediumTwigSlime, MonsterData {
        health: 26,
        moves: vec![
            vec![Moves::Attack(11)],
            vec![Moves::StatusCard(Card::Slimed)]
        ],
        starting_effects: vec![]
    });
    m.insert(Monsters::Byrdonis, MonsterData {
        health: 91,
        moves: vec![
            vec![Moves::Attack(3), Moves::Attack(3), Moves::Attack(3)],
            vec![Moves::Attack(16)]
        ],
        starting_effects: vec![
            Effect::Territorial(1)
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
    MediumTwigSlime,
    Byrdonis
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
    moves: Vec<Vec<Moves>>,
    starting_effects: Vec<Effect>
}

pub struct Enemy {
    pub id: u32,
    pub monster: Monsters,
    pub health: u32,
    pub block: u32,
    pub effects: Vec<Effect>,
    pub move_idx: usize,
    pub moves: Vec<Vec<Moves>>
}

impl Enemy {
    pub fn new(monster: Monsters) -> Self {
        Self {
            id: { let mut i = ENEMY_ID.lock().unwrap(); *i += 1; *i },
            health: MONSTERS[&monster].health,
            moves: MONSTERS[&monster].moves.clone(),
            effects: MONSTERS[&monster].starting_effects.clone(),
            monster,
            block: 0,
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
