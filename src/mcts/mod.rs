use std::{collections::HashMap, fmt::Display, sync::{LazyLock, Mutex}};

use rand::{RngExt, rngs::ThreadRng};

use crate::{EncounterOp, cards::library::Card, core::Encounter};

const EXPLORE_DECAY: f64 = 0.99975;

pub struct Search {
    pub nodes: HashMap<u32, ActionNode>,
    pub rng: ThreadRng,
    pub explore_rate: f64,
    pub last_node_id: u32
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
    pub position: Position,
    pub action: Option<Action>,
    pub expanded: bool,
    pub visited: bool,
    pub evals: u32,
    pub wins: u32,
}

impl Display for ActionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionNode").field("id", &self.id).field("action", &self.action).field("down", &self.down).field("wins", &self.wins).field("evals", &self.evals).field("position", &self.position).finish()
    }
}

impl Search {
    pub fn new() -> Self {
        Self {
            rng: rand::rng(),
            nodes: HashMap::new(),
            explore_rate: 1.,
            last_node_id: 0
        }
    }

    fn expand(&mut self, id: u32) {
        let node = self.nodes.get_mut(&id).unwrap();
        let actions = Self::get_all_actions(&node.encounter);

        let mut new = vec![];
        for action in actions {
            let mut child = ActionNode { id: self.last_node_id + 1, up: Some(node.id), down: vec![], position: Self::get_position(&node.encounter), encounter: node.encounter.clone(), action: Some(action), visited: false, expanded: false, wins: 0, evals: 0, };

            match &child.action {
                Some(Action::PlayAgainst(card)) => child.encounter.play(child.encounter.hand.iter().filter(|c| c.card == *card).nth(0).unwrap().id, child.encounter.enemies[0].id, vec![], &vec![]),
                Some(Action::PlaySelf(card)) => {
                    let mut discard = vec![];
                    if *card == Card::Survivor && let Some(d) = child.encounter.hand.iter().filter(|c| c.card != Card::Survivor).nth(0) {
                        discard.push(d.id);
                    }
                    child.encounter.play(child.encounter.hand.iter().filter(|c| c.card == *card).nth(0).unwrap().id, 0, discard, &vec![]);
                },
                Some(Action::NextTurn) => {
                    child.encounter.yield_turn();

                    if child.encounter.player.health == 0 || child.encounter.enemies[0].health == 0 {
                        continue;
                    }

                    child.encounter.end_turn();
                    child.encounter.begin_turn();
                }
                None => unreachable!()
            }

            self.last_node_id = child.id;
            
            node.down.push(child.id);
            new.push(child);
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
                    let child = self.nodes.get(*n);
                    return match child {
                        Some(c) => !c.expanded,
                        None => panic!()
                    }})
                .nth(0);
            if next_collapsed.is_some() {
                id = *next_collapsed.unwrap();
                //println!("Found new collapsed node {}", id);
                break;
            } else {

                let n = node.down.len();
                if n == 0 {
                    //println!("Found dead-end node");
                    return;
                }

                id = {
                    self.explore_rate *= EXPLORE_DECAY;

                    if self.rng.random_range(0. .. 1.) < self.explore_rate {
                        //println!("Traveling to random child node");
                        node.down[self.rng.random_range(0..n)]
                    } else {
                        //println!("Trying to find next best candidate");

                        // find the next collapsed node with the most wins
                        let mut candidates: Vec<(u32, u32)> = node.down.clone()
                            .iter()
                            .filter(|n| !self.nodes[&n].expanded)
                            .map(|n| (*n, self.nodes[&n].wins))
                            .collect();

                        // head back up to the root and try again
                        if candidates.len() == 0 {
                            //println!("Exhausted children, trying again");
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
    }

    fn evaluate(&mut self, id: u32) {
        let node = self.nodes.get_mut(&id).unwrap();

        let position = Self::get_position(&node.encounter);
        //println!("Eval: {} {:?}", node, position);

        let win = {
            let child_encounter = node.encounter.clone();
            Self::play_out(&mut self.rng, child_encounter)
        };

        // backpropagate the results so that parent.wins = children.sum(|c| c.wins) holds for all ancestors
        let mut this = node.id;
        loop {
            let node = self.nodes.get_mut(&this).unwrap();
            node.evals += 1;
            if win {
                node.wins += 1;
            }

            match node.up {
                Some(parent) => this = parent,
                None => break
            };
        }
    }
    
    fn play_out(rng: &mut ThreadRng, mut encounter: Encounter) -> bool {
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

            let actions = Self::get_all_actions(&encounter);
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
    
    pub fn get_position(encounter: &Encounter) -> Position {
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
        let mut actions = vec![Action::NextTurn];
        for card in &encounter.hand {
            if card.cost <= encounter.player.energy {
                actions.push(match card.card {
                    Card::SilentStrike => Action::PlayAgainst(card.card),
                    Card::SilentDefend => Action::PlaySelf(card.card),
                    Card::Neutralize => Action::PlayAgainst(card.card),
                    Card::Survivor => Action::PlaySelf(card.card),
                    _ => unreachable!()
                });
            }
        }
        
        actions
    }
}
