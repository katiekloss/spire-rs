use std::cmp::min;

use rand::RngExt;

use crate::{Effect, EncounterOp, RunOp, cards::CardType, monsters::Enemy, relics::{RelicImpl, Relics}};

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
        encounter.enemies.iter().map(|e| EncounterOp::ApplyTarget(e.id, Effect::Vulnerable(1))).collect()
    }),
    ..
};

pub static BLOOD_VIAL: RelicImpl = RelicImpl {
    combat_started: Some(|encounter| -> Vec<EncounterOp> {
        vec![EncounterOp::SetHealth(min(encounter.run.max_health, encounter.player.health + 2))]
    }),
    ..
};

pub static MANGO: RelicImpl = RelicImpl {
    picked_up: Some(|run| -> Vec<RunOp> {
        vec![RunOp::SetMaxHealth(run.max_health + 14)]
    }),
    ..
};

pub static MERCURY_HOURGLASS: RelicImpl = RelicImpl {
    turn_started: Some(|encounter| -> Vec<EncounterOp> {
        encounter.enemies.iter().filter(|e| e.health > 0).map(|e| EncounterOp::Damage(e.id, 3)).collect()
    }),
    ..
};

pub static ORNAMENTAL_FAN: RelicImpl = RelicImpl {
    combat_started: Some(|_| -> Vec<EncounterOp> {
        vec![EncounterOp::SetCounter(Relics::OrnamentalFan, 0)]
    }),
    card_played: Some(|card, _, encounter| -> Vec<EncounterOp> {
        let counter = encounter.run.relics[&Relics::OrnamentalFan];
        if matches!(card.typ, CardType::Attack) {
            if counter == 2 {
                return vec![EncounterOp::GainBlock(3), EncounterOp::SetCounter(Relics::OrnamentalFan, 0)];
            }
            else {
                return vec![EncounterOp::SetCounter(Relics::OrnamentalFan, counter + 1)];
            }
        }
        
        vec![]
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
        vec![EncounterOp::ApplySelf(Effect::Strength(1))]
    }),
    ..
};