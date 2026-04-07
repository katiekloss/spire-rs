#![cfg(test)]

use spire_rs::{Run, cards::library::Card, encounters::Encounter, monsters::{Enemy, Monsters}};

fn start_run() -> Run {
    let run = Run {
        deck: vec![],
        floor: 1,
        gold: 0,
        health: 100,
        relics: vec![],
    };

    run
}

#[test]
fn multiple_enemies_all_use_their_moves() {
    let run = start_run();
    let mut encounter = Encounter::new(&run);
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