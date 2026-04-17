use std::collections::HashMap;

use spire_rs::{Run, cards::{CardInstance, library::Card}, core::Encounter, mcts::{self, ActionNode, Search}, monsters::{Enemy, Monsters}};

fn start_encounter() -> Encounter {
    let mut run = Run {
        floor: 0,
        relics: HashMap::new(),
        health: 70,
        gold: 99,
        max_health: 70,
        deck: vec![],
    };

    for _ in 0..5 {
        run.deck.push(CardInstance::new(Card::SilentStrike));
        run.deck.push(CardInstance::new(Card::SilentDefend));
    }
    run.deck.push(CardInstance::new(Card::Survivor));
    run.deck.push(CardInstance::new(Card::Neutralize));

    let mut encounter = Encounter::new(run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));

    encounter
}

fn main() -> std::io::Result<()> {
    let mut encounter = start_encounter();
    encounter.begin_turn();

    let mut search = Search::new();

    search.nodes.insert(0, ActionNode {
        id: 0,
        up: None,
        down: vec![],
        action: None,
        position: Search::get_position(&encounter),
        encounter: encounter,
        visited: false,
        expanded: false,
        wins: 0,
        evals: 0,
    });

    for _ in 1..10_000 {
        search.next(0);
    }

    let mut uct = HashMap::new();
    for node in search.nodes.values() {
        uct.insert(node.id, (node.wins as f32 / node.evals as f32) + f32::sqrt(2.) * f32::sqrt((if node.up.is_some() { search.nodes[&node.up.unwrap()].evals } else { node.evals } as f32).ln() / node.evals as f32));
    }

    let mut ordered: Vec<(&u32, &f32)> = uct.iter().collect();
    ordered.sort_by(|a, b| b.1.total_cmp(a.1));

    for node in &ordered[0..10] {
        println!("{}: {}", node.1, search.nodes[&node.0]);
    }

    Ok(())
}
