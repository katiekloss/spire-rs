use std::cmp::min;

use rand::RngExt;

use crate::{EncounterOp, monsters::Enemy, relics::RelicImpl};

pub static RING_OF_THE_SNAKE: RelicImpl = RelicImpl {
    ..
};

pub static BLOOD_VIAL: RelicImpl = RelicImpl {
    combat_started: Some(|counter, encounter| -> u32 {
        encounter.player.health = min(encounter.run.max_health, encounter.player.health + 2);
        counter
    }),
    ..
};

pub static VAJRA: RelicImpl = RelicImpl {
    combat_started: Some(|_, encounter| -> u32 {
        encounter.player.effects.push(crate::Effect::Strength(1));
        0
    }),
    ..
};

pub static TINGSHA: RelicImpl = RelicImpl {
    card_discarded: Some(|_, encounter| -> Vec<EncounterOp> {
        let mut rng = rand::rng();
        let alive: Vec<&Enemy> = encounter.enemies.iter().filter(|e| e.health > 0).collect();
        let target = alive[rng.random_range(0..alive.len())];
        vec![EncounterOp::Damage(target.id, 3)]
    }),
    ..
};