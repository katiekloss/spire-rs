use std::{collections::HashMap, ops::RangeInclusive, sync::{LazyLock, Mutex}};

use rand::RngExt;

use crate::{Damageable, Effect, Effectable, EncounterOp, Target, Team, cards::library::Card, core::Encounter};

static ENEMY_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1)); // the player is 0
static MONSTERS: LazyLock<HashMap<Monsters, MonsterData>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Monsters::FuzzyWurmCrawler, MonsterData {
        health: 55..=57,
        moves: EnemyMoves::Static(vec![
            vec![Moves::Attack(4)],
            vec![Moves::Attack(4)],
            vec![Moves::Buff(Effect::Strength(7))]]),
        starting_effects: vec![]
    });
    m.insert(Monsters::SmallLeafSlime, MonsterData {
        health: 11..=15,
        moves: EnemyMoves::Static(vec![
            vec![Moves::Attack(3)],
            vec![Moves::StatusCard(Card::Slimed)]
        ]),
        starting_effects: vec![]
    });
    m.insert(Monsters::MediumLeafSlime, MonsterData {
        health: 32..=35,
        moves: EnemyMoves::Static(vec![
            vec![Moves::Attack(8)],
            vec![Moves::StatusCard(Card::Slimed)]
        ]),
        starting_effects: vec![]
    });
    m.insert(Monsters::SmallTwigSlime, MonsterData {
        health: 7..=11,
        moves: EnemyMoves::Static(vec![
            vec![Moves::Attack(4)]
        ]),
        starting_effects: vec![]
    });
    m.insert(Monsters::MediumTwigSlime, MonsterData {
        health: 26..=28,
        moves: EnemyMoves::Static(vec![
            vec![Moves::Attack(11)],
            vec![Moves::StatusCard(Card::Slimed)]
        ]),
        starting_effects: vec![]
    });
    m.insert(Monsters::Byrdonis, MonsterData {
        health: 91..=94,
        moves: EnemyMoves::Static(vec![
            vec![Moves::Attack(3), Moves::Attack(3), Moves::Attack(3)],
            vec![Moves::Attack(16)]
        ]),
        starting_effects: vec![
            Effect::Territorial(1)
        ]
    });
    m.insert(Monsters::BygoneEffigy, MonsterData {
        health: 127..=127,
        moves: EnemyMoves::Custom(|encounter| {
            match encounter.turn {
                1 => vec![],
                2 => vec![EncounterOp::ApplyTarget(encounter.enemies[0].id, Effect::Strength(10))],
                3 => vec![],
                _ => vec![EncounterOp::AttackPlayer(encounter.enemies[0].id, 15)]
            }
        }),
        starting_effects: vec![]
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
    Byrdonis,
    BygoneEffigy
}

#[derive(Clone, Debug)]
pub enum Moves {
    Attack(u32),
    Buff(Effect),
    Debuff(Effect),
    StatusCard(Card)
}

pub struct MonsterData {
    health: RangeInclusive<u32>,
    moves: EnemyMoves,
    starting_effects: Vec<Effect>
}

type CustomMoveHandler = fn(&Encounter) -> Vec<EncounterOp>;

#[derive(Clone)]
pub enum EnemyMoves {
    Static(Vec<Vec<Moves>>),
    Custom(CustomMoveHandler)
}

pub struct Enemy {
    pub id: u32,
    pub monster: Monsters,
    pub health: u32,
    pub block: u32,
    pub effects: Vec<Effect>,
    pub move_idx: usize,
    pub moves: EnemyMoves,
}

impl Enemy {
    pub fn new(monster: Monsters) -> Self {
        let mut rng = rand::rng();
        let def = &MONSTERS[&monster];
        Self {
            id: { let mut i = ENEMY_ID.lock().unwrap(); *i += 1; *i },
            health: rng.random_range(def.health.clone()),
            moves: def.moves.clone(),
            effects: def.starting_effects.clone(),
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
