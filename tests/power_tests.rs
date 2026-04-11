#![cfg(test)]

use std::collections::HashMap;

use spire_rs::{Run, cards::{CardInstance, library::Card}, core::Encounter, get_card, powers::defs::AFTERIMAGE};

fn start_run() -> Run {
    let run = Run {
        deck: vec![],
        floor: 1,
        gold: 0,
        health: 100,
        max_health: 100,
        relics: HashMap::new(),
    };

    run
}

#[test]
fn afterimage() {
    let mut run = start_run();
    run.deck.push(CardInstance::new(Card::SilentDefend));

    let mut encounter = Encounter::new(&mut run);
    encounter.player.effects.push(spire_rs::Effect::Custom(&AFTERIMAGE));

    encounter.begin_turn();
    encounter.play(get_card!(Card::SilentDefend, encounter.hand).unwrap().id, 0, vec![], &vec![]);

    // Defend + 1 from Afterimage
    assert_eq!(encounter.player.block, 6);
}