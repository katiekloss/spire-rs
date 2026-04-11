#![cfg(test)]

use std::{assert_matches, collections::HashMap};

use spire_rs::{Effect, Run, cards::{CardInstance, library::Card}, core::Encounter, get_card, monsters::{Enemy, Monsters, Moves}};

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
fn multiple_enemies_all_use_their_moves() {
    let mut run = start_run();
    let mut encounter = Encounter::new(&mut run);
    encounter.enemies.push(Enemy::new(Monsters::SmallLeafSlime));
    encounter.enemies.push(Enemy::new(Monsters::MediumLeafSlime));

    encounter.begin_turn();
    encounter.yield_turn();

    assert_eq!(encounter.player.health, 89);

    encounter.end_turn();
    encounter.begin_turn();
    encounter.yield_turn();

    assert_eq!(encounter.draw_pile.iter().filter(|c| c.card == Card::Slimed).count(), 2);
}

#[test]
fn power_cards_vanish_on_play() {
    let mut run = start_run();
    run.deck.push(CardInstance::new(Card::Afterimage));
    let mut encounter = Encounter::new(&mut run);
    encounter.begin_turn();
    encounter.play(get_card!(Card::Afterimage, encounter.hand).unwrap().id, 0, vec![], &vec![]);

    assert_eq!(encounter.discard_pile.len(), 0);
    assert_eq!(encounter.exhaust_pile.len(), 0);
}

#[test]
fn effects_tick_on_turn_start() {
    let mut run = start_run();
    let mut encounter = Encounter::new(&mut run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));
    encounter.enemies[0].effects.push(Effect::Vulnerable(2));
    encounter.player.effects.push(Effect::Vulnerable(2));

    encounter.begin_turn();

    if let Effect::Vulnerable(v) = encounter.player.effects[0] {
        assert_eq!(1, v);
    } else {
        panic!();
    }

    if let Effect::Vulnerable(v) = encounter.enemies[0].effects[0] {
        assert_eq!(1, v);
    } else {
        panic!();
    }

    encounter.yield_turn();
    encounter.end_turn();
    encounter.begin_turn();
    assert_eq!(0, encounter.player.effects.len());
    assert_eq!(0, encounter.enemies[0].effects.len());
}

#[test]
fn vulnerable_multiplies_attack_damage() {
    let mut run = start_run();
    run.deck.push(CardInstance::new(Card::SilentStrike));
    let mut encounter = Encounter::new(&mut run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));
    let health = encounter.enemies[0].health;
    encounter.enemies[0].effects.push(Effect::Vulnerable(2));
    encounter.player.effects.push(Effect::Vulnerable(2));
    encounter.begin_turn();

    assert_matches!(encounter.get_enemy_intent(&encounter.enemies[0])[..], [Moves::Attack(6)]);
    encounter.play(get_card!(Card::SilentStrike, encounter.hand).unwrap().id, encounter.enemies[0].id, vec![], &vec![]);

    assert_eq!(encounter.enemies[0].health, health - 9);
    encounter.run.health = 0;
}