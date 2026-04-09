#![cfg(test)]

use spire_rs::{Run, encounters::{self, Encounter}, relics::Relics};

fn start_run() -> Run {
    let run = Run {
        deck: vec![],
        floor: 1,
        gold: 0,
        health: 100,
        max_health: 100,
        relics: vec![],
    };

    run
}

#[test]
fn blood_vial_adds_health() {
    let mut run = start_run();
    run.health = 90;
    run.relics.push(Relics::BloodVial);
    let mut encounter = Encounter::new(&run);
    encounter.begin_turn();

    assert_eq!(encounter.player.health, 92);
}

#[test]
fn blood_vial_doesnt_exceed_max_health() {
    let mut run = start_run();
    run.health = 100;
    run.relics.push(Relics::BloodVial);
    let mut encounter = Encounter::new(&run);
    encounter.begin_turn();

    assert_eq!(encounter.player.health, 100);
}