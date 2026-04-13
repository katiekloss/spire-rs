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
    let mut encounter = Encounter::new(run);
    encounter.begin_turn();

    assert_eq!(encounter.player.health, 92);
}

#[test]
fn blood_vial_doesnt_exceed_max_health() {
    let mut run = start_run();
    run.health = 100;
    run.relics.insert(Relics::BloodVial, 0);
    let mut encounter = Encounter::new(run);
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

    let mut encounter = Encounter::new(run);
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

    let mut encounter = Encounter::new(run);
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

#[test]
fn mango_raises_max_hp() {
    let mut run = start_run();
    run.health = 100;
    run.max_health = 100;
    run.pickup_relic(Relics::Mango);
    assert_eq!(run.max_health, 114);
    assert_eq!(run.health, 114);
}

#[test]
fn ornamental_fan_applies_block() {
    let mut run = start_run();
    for _ in 0..7 {
        run.deck.push(CardInstance::new(Card::SilentStrike));
    }
    run.pickup_relic(Relics::OrnamentalFan);
    run.pickup_relic(Relics::RingOfTheSnake);

    let mut encounter = Encounter::new(run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));
    encounter.enemies[0].health = 100;

    encounter.begin_turn();
    encounter.player.energy = 20;
    encounter.play(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id, vec![], &vec![]);
    assert_eq!(encounter.player.block, 0);

    encounter.play(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id, vec![], &vec![]);
    assert_eq!(encounter.player.block, 0);

    encounter.play(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id, vec![], &vec![]);
    assert_eq!(encounter.player.block, 3);

    encounter.play(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id, vec![], &vec![]);
    assert_eq!(encounter.player.block, 3);

    encounter.play(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id, vec![], &vec![]);
    assert_eq!(encounter.player.block, 3);

    encounter.play(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id, vec![], &vec![]);
    assert_eq!(encounter.player.block, 6);
}

#[test]
fn ornamental_fan_ignores_skills() {
    let mut run = start_run();
    run.deck.push(CardInstance::new(Card::SilentDefend));
    run.pickup_relic(Relics::OrnamentalFan);

    let mut encounter = Encounter::new(run);
    encounter.begin_turn();
    encounter.play(get_card!(Card::SilentDefend, encounter.hand).unwrap().id, 0, vec![], &vec![]);
    
    let run = encounter.end();
    assert_eq!(run.relics[&Relics::OrnamentalFan], 0);
}