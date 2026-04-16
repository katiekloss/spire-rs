#![allow(static_mut_refs)]

use core::panic;
use std::{collections::HashMap, fmt::Display, sync::{LazyLock, Mutex}};

use rand::{RngExt, rngs::ThreadRng};
use spire_rs::{EncounterOp, Run, cards::{CardInstance, library::Card}, core::Encounter, monsters::{Enemy, Monsters}};

static NODE_IDS: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
static mut NODE_STATS: LazyLock<HashMap<u32, NodeStats>> = LazyLock::new(|| HashMap::new());

const EXPLORE_DECAY: f64 = 0.9999975;

static EXPLORE_RATE: Mutex<f64> = Mutex::new(1.);

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    PlaySelf(Card),
    PlayAgainst(Card)
}

#[derive(Debug)]
pub struct NodeStats {
    pub parent: Option<u32>,
    pub wins: u32,
    pub evals: u32,
    id: u32
}

#[derive(Clone, Debug)]
pub enum Order {
    Evaluate,
    Playout
}

#[derive(Clone, Debug)]
pub enum EvalResult {
    Success,
    Failure
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Position {
    pub turn: u32,
    pub hand: Vec<Card>,
    pub incoming_damage: u32,
    pub block: u32
}

#[derive(Clone)]
pub struct ActionNode {
    pub id: u32,
    pub down: Vec<ActionNode>,
    pub encounter: Encounter,
    pub action: Option<Action>,
    pub expanded: bool,
    pub visited: bool,
    pub evaluated: bool,
}

impl Display for ActionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionNode").field("id", &self.id).field("action", &self.action).field("down", &self.down.len()).finish()
    }
}

fn start_encounter() -> Encounter {
    let mut run = Run {
        floor: 0,
        relics: HashMap::new(),
        health: 5,
        gold: 99,
        max_health: 5,
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
    let mut rng = rand::rng();
    let mut encounter = start_encounter();
    encounter.begin_turn();
    
    let mut root_node = ActionNode {
        id: 0,
        down: vec![],
        action: None,
        encounter: encounter.clone(),
        visited: false,
        expanded: false,
        evaluated: false
    };

    unsafe { NODE_STATS.insert(root_node.id, NodeStats { id: root_node.id, parent: None, wins: 0, evals: 0 }) };
    
    visit(&mut rng, &mut root_node);
    unsafe {
        let mut nodes: Vec<&NodeStats> = NODE_STATS.values().collect();
        nodes.sort_by(|a, b| b.evals.cmp(&a.evals));
        println!("{:?}", &nodes[0..10]);
    }

    Ok(())
}

pub fn expand(node: &mut ActionNode) {
    let actions = get_all_actions(&node.encounter);

    let mut ids = NODE_IDS.lock().unwrap();
    for action in actions {
        let child = ActionNode { id: *ids, down: vec![], encounter: node.encounter.clone(), action: Some(action), visited: false, expanded: false, evaluated: false };
        unsafe { NODE_STATS.insert(child.id, NodeStats { id: child.id, parent: Some(node.id), wins: 0, evals: 0 }); };
        node.down.push(child);
        *ids += 1;
    }

    node.expanded = true;
}

pub fn visit(rng: &mut ThreadRng, mut node: &mut ActionNode) {
    for i in 1..1000 {
        node.visited = true;

        if !node.expanded {
            expand(node);
        }

        evaluate(rng, node);

        if !node.visited {
            node.visited = true;
        }

        println!("Visited {}", node);

        let n = node.down.len();
        node = {
            let mut explore_rate = EXPLORE_RATE.lock().unwrap();
            *explore_rate *= EXPLORE_DECAY; // but ELEGANCE

            if rng.random_range(0. .. 1.) < *explore_rate {
                &mut node.down[rng.random_range(0..n)]
            } else {
                unsafe { node.down.sort_by(|a, b| NODE_STATS[&b.id].wins.cmp(&NODE_STATS[&a.id].wins)); }
                node.down.iter_mut().filter(|n| !n.expanded).nth(0).unwrap()
            }
        };
    }
}

/// Applies the node's action to its state and then plays it out from there
pub fn evaluate(rng: &mut ThreadRng, node: &mut ActionNode) {
    match node.action {
        Some(Action::PlayAgainst(card)) => node.encounter.play(node.encounter.hand.iter().filter(|c| c.card == card).nth(0).unwrap().id, node.encounter.enemies[0].id, vec![], &vec![]),
        Some(Action::PlaySelf(card)) => node.encounter.play(node.encounter.hand.iter().filter(|c| c.card == card).nth(0).unwrap().id, 0, vec![], &vec![]),
        None => return
    }

    let position = get_position(&node.encounter);

    let mut wins = 0;

    for _ in 0..100 {
        let child_encounter = node.encounter.clone();
        if play_out(rng, child_encounter) {
            wins += 1;
        } else {
        }
    }

    node.evaluated = true;

    unsafe {
        let mut this = node.id;
        loop {
            let root = NODE_STATS.get(&0).unwrap();
            let stats = NODE_STATS.get_mut(&this).unwrap();

            stats.evals += 1;
            if wins > 50 {
                stats.wins += 1;
            }

            match stats.parent {
                Some(parent) => this = parent,
                None => break
            };
        }
    }
}

pub fn play_out(rng: &mut ThreadRng, mut encounter: Encounter) -> bool {
    loop {
        if encounter.player.energy == 0 {
            encounter.yield_turn();

            if encounter.player.health == 0 {
                return false;
            } else if encounter.enemies[0].health == 0 {
                return true;
            }

            encounter.end_turn();
            encounter.begin_turn();
        }

        let actions = get_all_actions(&encounter);
        let action = &actions[rng.random_range(0..actions.len())];

        match action {
            Action::PlayAgainst(card) => encounter.play(encounter.hand.iter().filter(|c| c.card == *card).nth(0).unwrap().id, encounter.enemies[0].id, vec![], &vec![]),
            Action::PlaySelf(card) => encounter.play(encounter.hand.iter().filter(|c| c.card == *card).nth(0).unwrap().id, 0, vec![], &vec![])
        };

        if encounter.player.health == 0 {
            return false;
        } else if encounter.enemies[0].health == 0 {
            return true;
        }
    }
}

// // returns (successful_position, won)
// pub fn play_from(rng: &mut ThreadRng, mut encounter: Encounter, order: Order) -> PlayResult {
//     if encounter.player.health == 0 {
//         return PlayResult::Lost;
//     } else if encounter.enemies[0].health == 0 {
//         return PlayResult::Won;
//     }

//     if encounter.player.energy == 0 {
//         encounter.yield_turn();

//         if encounter.player.health == 0 {
//             return PlayResult::Lost;
//         } else if encounter.enemies[0].health == 0 {
//             return PlayResult::Won;
//         }

//         encounter.end_turn();
//         encounter.begin_turn();
//         println!("Turn {}", encounter.turn);
//     }

//     let position = get_position(&encounter);

//     let actions = get_all_actions(&encounter);
//     let our_action = {
//         let explore_rate = EXPLORE_RATE.lock().unwrap();

//         if matches!(order, Order::Playout) || rng.random_range(0. .. 1.) < *explore_rate {
//             &actions[rng.random_range(0..actions.len())]
//         } else {
//             recall_closest_position(&position)
//         }
//     };

//     let mut encounter = encounter.clone();
//     match our_action {
//         Action::PlayAgainst(card) => encounter.play(encounter.hand.iter().filter(|c| c.card == *card).nth(0).unwrap().id, encounter.enemies[0].id, vec![], &vec![]),
//         Action::PlaySelf(card) => encounter.play(encounter.hand.iter().filter(|c| c.card == *card).nth(0).unwrap().id, 0, vec![], &vec![])
//     };

//     if encounter.enemies[0].health == 0 {
//         return PlayResult::Won; // hmm.
//     }

//     let this_position = get_position(&encounter);

//     if let Order::Evaluate = order {
//         let mut wins = 0;
//         for i in 1..5 {
//             println!("Eval iteration {} from {:?}", i, &this_position);
//             let new_encounter = encounter.clone();
//             let result = play_from(rng, new_encounter, Order::Playout);

//             match result {
//                 PlayResult::Failure | PlayResult::Lost => println!("Died in iteration {} from {:?}", i, &this_position),
//                 PlayResult::Success | PlayResult::Won => {
//                     wins += 1;
//                     println!("Won in iteration {} from {:?}", i, &this_position);
//                 }
//             }
//         }

//         if wins > 2 {
//             return PlayResult::Success;
//         } else {
//             return PlayResult::Failure;
//         }
//     } else {
//         println!("Continuing play out from {:?}", this_position);
//         return play_from(rng, encounter, Order::Playout);
//     }
// }

fn get_position(encounter: &Encounter) -> Position {
    let incoming_damage = match encounter.get_enemy_intent(&encounter.enemies[0])[0] {
        EncounterOp::AttackPlayer(_, dmg) => Encounter::query_attack_damage(&encounter.enemies[0], &encounter.player, dmg),
        EncounterOp::Damage(_, dmg) => dmg,
        _ => 0
    };

    Position {
        turn: encounter.turn,
        block: encounter.player.block,
        hand: encounter.hand.iter().map(|c| c.card).collect(),
        incoming_damage
    }
}

fn get_all_actions(encounter: &Encounter) -> Vec<Action> {
    let mut actions = vec![];
    for card in &encounter.hand {
        actions.push(match card.card {
            Card::SilentStrike => Action::PlayAgainst(card.card),
            Card::SilentDefend => Action::PlaySelf(card.card),
            Card::Neutralize => Action::PlayAgainst(card.card),
            Card::Survivor => Action::PlaySelf(card.card),
            _ => unreachable!()
        });
    }
    
    actions
}
