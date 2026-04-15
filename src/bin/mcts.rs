use std::{cell::{Cell, UnsafeCell}, collections::HashMap, sync::{LazyLock, Mutex, RwLock}};

use rand::{RngExt, rngs::ThreadRng};
use spire_rs::{EncounterOp, Run, cards::{CardInstance, library::Card}, core::Encounter, monsters::{Enemy, Monsters}};

const EXPLORE_DECAY: f64 = 0.99999975;

static EXPLORE_RATE: Mutex<f64> = Mutex::new(1.);
static MEMORY: LazyLock<Memory> = LazyLock::new(|| Memory { states: HashMap::new()});


#[derive(Debug)]
pub enum Action {
    PlaySelf(Card),
    PlayAgainst(Card)
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
    
    for i in 0..10 {
        let mut encounter = start_encounter();
        encounter.begin_turn();
        evaluate(&mut rng, encounter);
        
        let mut rate = EXPLORE_RATE.lock().unwrap();
        *rate = *rate * EXPLORE_DECAY;
    }

    Ok(())
}

pub fn evaluate(rng: &mut ThreadRng, mut encounter: Encounter) -> EvalResult {
    let position = get_position(&encounter);
    let actions = get_all_actions(&encounter);

    // choose by epsilon greedy
    let action = {
        let explore_rate = EXPLORE_RATE.lock().unwrap();

        if rng.random_range(0. .. 1.) < *explore_rate {
            &actions[rng.random_range(0..actions.len())]
        } else {
            recall_by_closest_position(&position)
        }
    };

    let mut wins = 0;

    for _ in 0..10 {
        let child_encounter = encounter.clone();
        if play_out(rng, child_encounter) {
            println!("Won by {:?} from {:?}", action, position);
            wins += 1;
        } else {
            println!("Lost by {:?} from {:?}", action, position);
        }
    }

    return if wins > 5 {
        println!("Taking {:?} from {:?} is successful", action, position);
        EvalResult::Success
    } else {
        println!("Taking {:?} from {:?} is a failure", action, position);
        EvalResult::Failure
    }
}

pub fn play_out(rng: &mut ThreadRng, mut encounter: Encounter) -> bool {
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

    return if encounter.player.health == 0 {
        false
    } else if encounter.enemies[0].health == 0 {
        true
    } else {
        play_out(rng, encounter)
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

fn recall_by_closest_position(position: &Position) -> &Action {
    &MEMORY.states.get(position).expect("Unknown position")
}

pub struct Memory {
    pub states: HashMap<Position, Action>
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Position {
    pub hand: Vec<Card>,
    pub incoming_damage: u32,
    pub block: u32
}