#![cfg(test)]

use spire_rs::{Run, cards::{Card, CardInstance}, encounters::Encounter, get_card, map::MapGenerator, monsters::{Enemy, Monsters}};

fn start_run() -> Run {
    let run = Run {
        deck: vec![],
        floor: 1,
        gold: 0,
        health: 70,
        relics: vec![],
        current_room: MapGenerator::generate()
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
    assert_eq!(encounter.enemies[0].health, 55);

    encounter.begin_turn();

    encounter.player.effects.push(spire_rs::Effect::Strength(1));
    encounter.play_by_id_with_target(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id);
    assert_eq!(encounter.enemies[0].health, 48);

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
    assert_eq!(encounter.enemies[0].health, 55);

    encounter.begin_turn();

    encounter.player.effects.push(spire_rs::Effect::Weak(1));
    encounter.play_by_id_with_target(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id);
    assert_eq!(encounter.enemies[0].health, 51);

    encounter.enemies[0].effects.push(spire_rs::Effect::Weak(1));
    encounter.yield_turn();
    assert_eq!(encounter.player.health, 67);
}