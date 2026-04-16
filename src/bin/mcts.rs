use std::{collections::HashMap, fmt::Display, hash::Hash, sync::{LazyLock, Mutex}};

use rand::{RngExt, rngs::ThreadRng};
use spire_rs::{EncounterOp, Run, cards::{CardInstance, library::Card}, core::Encounter, monsters::{Enemy, Monsters}};

static NODE_IDS: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));

const EXPLORE_DECAY: f64 = 0.99975;

static EXPLORE_RATE: Mutex<f64> = Mutex::new(1.);

pub struct Search {
    pub nodes: HashMap<u32, ActionNode>,
    pub rng: ThreadRng,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    PlaySelf(Card),
    PlayAgainst(Card),
    NextTurn
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
    pub up: Option<u32>,
    pub down: Vec<u32>,
    pub encounter: Encounter,
    pub action: Option<Action>,
    pub expanded: bool,
    pub visited: bool,
    pub evals: u32,
    pub wins: u32,
}

impl Display for ActionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionNode").field("id", &self.id).field("action", &self.action).field("down", &self.down).field("wins", &self.wins).field("evals", &self.evals).finish()
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
    let mut encounter = start_encounter();
    encounter.begin_turn();

    let mut search = Search {
        nodes: HashMap::new(),
        rng: rand::rng()
    };

    search.nodes.insert(0, ActionNode {
        id: 0,
        up: None,
        down: vec![],
        action: None,
        encounter: encounter,
        visited: false,
        expanded: false,
        wins: 0,
        evals: 0
    });

    for i in 1..1_000 {
        search.next(0);
    }

    let mut nodes: Vec<&mut ActionNode> = search.nodes.values_mut().collect();
    nodes.sort_by(|a, b| b.evals.cmp(&a.evals));
    for node in &nodes[0..10] {
        println!("{}", **node);
    }

    Ok(())
}

impl Search {

    pub fn expand(&mut self, id: u32) {
        let node = self.nodes.get_mut(&id).unwrap();
        let actions = get_all_actions(&node.encounter);

        let mut ids = NODE_IDS.lock().unwrap();
        let mut new = vec![];
        for action in actions {
            let mut child = ActionNode { id: *ids, up: Some(node.id), down: vec![], encounter: node.encounter.clone(), action: Some(action), visited: false, expanded: false, wins: 0, evals: 0 };

            match &child.action {
                Some(Action::PlayAgainst(card)) => child.encounter.play(child.encounter.hand.iter().filter(|c| c.card == *card).nth(0).unwrap().id, child.encounter.enemies[0].id, vec![], &vec![]),
                Some(Action::PlaySelf(card)) => child.encounter.play(child.encounter.hand.iter().filter(|c| c.card == *card).nth(0).unwrap().id, 0, vec![], &vec![]),
                Some(Action::NextTurn) => {
                    child.encounter.yield_turn();

                    if child.encounter.player.health == 0 || child.encounter.enemies[0].health == 0 {
                        node.expanded = true;
                        return;
                    }

                    child.encounter.end_turn();
                    child.encounter.begin_turn();
                }
                None => unreachable!()
            }
            
            node.down.push(child.id);
            new.push(child);
            *ids += 1;
        }

        node.expanded = true;

        for node in new {
            self.nodes.insert(node.id, node);
        }
    }

    pub fn next(&mut self, mut id: u32) {
        if !self.nodes[&id].expanded {
            self.expand(id);
        }

        loop {
            let node = &self.nodes[&id];
            let next_collapsed = node.down.iter()
                .filter(|n| {
                    println!("Checking node {}", *n);
                    let child = self.nodes.get(*n);
                    return match child {
                        Some(c) => !c.expanded,
                        None => panic!()
                    }})
                .nth(0);
            if next_collapsed.is_some() {
                id = *next_collapsed.unwrap();
                println!("Found new collapsed node {}", id);
                break;
            } else {

                let n = node.down.len();
                if n == 0 {
                    println!("Found dead-end node");
                    return;
                }

                id = {
                    let mut explore_rate = EXPLORE_RATE.lock().unwrap();
                    *explore_rate *= EXPLORE_DECAY;

                    if self.rng.random_range(0. .. 1.) < *explore_rate {
                        println!("Traveling to random child node");
                        node.down[self.rng.random_range(0..n)]
                    } else {
                        println!("Trying to find next best candidate");
                        let mut candidates: Vec<(u32, u32)> = node.down.clone()
                            .iter()
                            .filter(|n| !self.nodes[&n].expanded)
                            .map(|n| (*n, self.nodes[&n].wins))
                            .collect();

                        // head back up to the root and try again
                        if candidates.len() == 0 {
                            println!("Exhausted children, trying again");
                            return;
                        }

                        candidates.sort_by(|a, b| b.1.cmp(&a.1));
                        candidates[0].0
                    }
                };
            }
        }

        self.expand(id);
        self.evaluate(id);

        //println!("Visited {}", node);

        
    }

    /// Applies the node's action to its state and then plays it out from there
    pub fn evaluate(&mut self, id: u32) {
        let node = self.nodes.get_mut(&id).unwrap();

        let position = get_position(&node.encounter);
        println!("Eval: {} {:?}", node, position);
        let mut encounter = node.encounter.clone();

        let mut wins = 0;

        for _ in 0..100 {
            let child_encounter = encounter.clone();
            if play_out(&mut self.rng, child_encounter) {
                wins += 1;
            }
        }

        // backpropagate the results so that parent.wins = children.sum(|c| c.wins) for all ancestors
        let mut this = node.id;
        loop {
            let node = self.nodes.get_mut(&this).unwrap();
            node.evals += 1;
            if wins > 50 {
                node.wins += 1;
            }

            match node.up {
                Some(parent) => this = parent,
                None => break
            };
        }
    }
}

pub fn play_out(rng: &mut ThreadRng, mut encounter: Encounter) -> bool {
    loop {
        if encounter.player.energy == 0 || encounter.hand.len() == 0 {
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
            Action::PlaySelf(card) => encounter.play(encounter.hand.iter().filter(|c| c.card == *card).nth(0).unwrap().id, 0, vec![], &vec![]),
            Action::NextTurn => {
                encounter.yield_turn();

                if encounter.player.health == 0 {
                    return false;
                } else if encounter.enemies[0].health == 0 {
                    return true;
                }

                encounter.end_turn();
                encounter.begin_turn();
            }
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
        actions.push(Action::NextTurn);
    }
    
    actions
}
