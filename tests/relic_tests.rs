#![cfg(test)]

use std::collections::HashMap;

use spire_rs::{Run, cards::{CardInstance, library::Card}, core::Encounter, get_card, monsters::{Enemy, Monsters}, relics::Relics};

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
fn blood_vial_adds_health() {
    let mut run = start_run();
    run.health = 90;
    run.relics.insert(Relics::BloodVial, 0);
    let mut encounter = Encounter::new(&mut run);
    encounter.begin_turn();

    assert_eq!(encounter.player.health, 92);
}

#[test]
fn blood_vial_doesnt_exceed_max_health() {
    let mut run = start_run();
    run.health = 100;
    run.relics.insert(Relics::BloodVial, 0);
    let mut encounter = Encounter::new(&mut run);
    encounter.begin_turn();

    assert_eq!(encounter.player.health, 100);
}

#[test]
fn tingsha_damages_enemies() {
    let mut run = start_run();
    run.health = 100;
    run.deck.push(CardInstance::new(Card::Survivor));
    run.deck.push(CardInstance::new(Card::SilentDefend));
    run.relics.insert(Relics::Tingsha, 0);

    let mut encounter = Encounter::new(&mut run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));
    let starting_health = encounter.enemies[0].health;
    
    encounter.begin_turn();
    encounter.play(get_card!(Card::Survivor, encounter.hand).unwrap().id, 0, vec![get_card!(Card::SilentDefend, encounter.hand).unwrap().id], &vec![]);

    assert_ne!(encounter.enemies[0].health, starting_health);
}

#[test]
fn mercury_hourglass_damages_enemies() {
    let mut run = start_run();
    run.relics.insert(Relics::MercuryHourglass, 0);

    let mut encounter = Encounter::new(&mut run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));

    let starting_health = encounter.enemies[0].health;
    encounter.begin_turn();
    assert_ne!(encounter.enemies[0].health, starting_health);

    let starting_health = encounter.enemies[0].health;
    encounter.yield_turn();
    encounter.end_turn();
    encounter.begin_turn();
    assert_ne!(encounter.enemies[0].health, starting_health);
}