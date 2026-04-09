use std::cmp::min;

use crate::relics::RelicImpl;

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