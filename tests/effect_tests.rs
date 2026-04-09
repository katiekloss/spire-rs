#![cfg(test)]

use spire_rs::{Effect, Run, cards::{CardInstance, library::Card}, encounters::Encounter, get_card, monsters::{Enemy, Monsters}};

fn start_run() -> Run {
    let run = Run {
        deck: vec![],
        floor: 1,
        gold: 0,
        health: 70,
        max_health: 70,
        relics: vec![],
    };

    run
}

#[test]
pub fn applying_strength_increases_damage() {
    let mut run = start_run();
    run.deck.push(CardInstance::new(Card::SilentStrike));
    run.deck.push(CardInstance::new(Card::SilentStrike));

    let mut encounter = Encounter::new(&run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));
    let health = encounter.enemies[0].health;

    encounter.begin_turn();

    encounter.player.effects.push(spire_rs::Effect::Strength(1));
    encounter.play(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id, vec![], &vec![]);
    assert_eq!(encounter.enemies[0].health, health - 7);

    encounter.enemies[0].effects.push(spire_rs::Effect::Strength(1));
    encounter.yield_turn();
    assert_eq!(encounter.player.health, 65);
}

#[test]
pub fn applying_weak_reduces_damage() {
    let mut run = start_run();
    run.deck.push(CardInstance::new(Card::SilentStrike));
    run.deck.push(CardInstance::new(Card::SilentStrike));

    let mut encounter = Encounter::new(&run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));
    let health = encounter.enemies[0].health;

    encounter.begin_turn();

    encounter.player.effects.push(spire_rs::Effect::Weak(1));
    encounter.play(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id, vec![], &vec![]);
    assert_eq!(encounter.enemies[0].health, health - 4);

    encounter.enemies[0].effects.push(spire_rs::Effect::Weak(1));
    encounter.yield_turn();
    assert_eq!(encounter.player.health, 67);
}

#[test]
pub fn applying_territorial_applies_strength() {
    let run = start_run();
    let mut encounter = Encounter::new(&run);
    encounter.enemies.push(Enemy::new(Monsters::Byrdonis));
    
    encounter.begin_turn();
    encounter.yield_turn();
    encounter.end_turn();

    assert_eq!(encounter.enemies[0].effects.iter().filter(|fx| **fx == Effect::Strength(1)).count(), 1);

    encounter.begin_turn();
    encounter.yield_turn();
    encounter.end_turn();

    assert_eq!(encounter.enemies[0].effects.iter().filter(|fx| **fx == Effect::Strength(1)).count(), 2);
}
