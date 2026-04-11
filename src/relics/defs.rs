use std::cmp::min;

use rand::RngExt;

use crate::{Effect, EncounterOp, monsters::Enemy, relics::RelicImpl};

pub static RING_OF_THE_SNAKE: RelicImpl = RelicImpl {
    ..
};

pub static ANCHOR: RelicImpl = RelicImpl {
    combat_started: Some(|_| -> Vec<EncounterOp> {
        vec![EncounterOp::GainBlock(10)]
    }),
    ..
};

pub static BAG_OF_MARBLES: RelicImpl = RelicImpl {
    combat_started: Some(|encounter| -> Vec<EncounterOp> {
        encounter.enemies.iter().map(|e| EncounterOp::TargetPush(e.id, Effect::Vulnerable(1))).collect()
    }),
    ..
};

pub static BLOOD_VIAL: RelicImpl = RelicImpl {
    combat_started: Some(|encounter| -> Vec<EncounterOp> {
        vec![EncounterOp::SetHealth(min(encounter.run.max_health, encounter.player.health + 2))]
    }),
    ..
};

pub static MERCURY_HOURGLASS: RelicImpl = RelicImpl {
    turn_started: Some(|encounter| -> Vec<EncounterOp> {
        encounter.enemies.iter().filter(|e| e.health > 0).map(|e| EncounterOp::Damage(e.id, 3)).collect()
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

pub static VAJRA: RelicImpl = RelicImpl {
    combat_started: Some(|_| -> Vec<EncounterOp> {
        vec![EncounterOp::SelfPush(Effect::Strength(1))]
    }),
    ..
};