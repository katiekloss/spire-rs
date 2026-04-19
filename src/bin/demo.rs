#![feature(hash_map_macro)]

use core::panic;
use std::{fs::{File, OpenOptions}, hash_map, sync::{LazyLock, Mutex}};

use log::{debug, info, trace};
use spire_rs::{Run, cards::{CardInstance, CardType, library::{CARDS, Card}}, core::Encounter, get_card, map::{MapGenerator, MapRoom, RoomType}, mcts::{Action, ActionNode, Search}, monsters::Enemy, relics::Relics};
use std_logger::Config;

fn main() {
    Config::logfmt().init();

    let mut run = Run {
        floor: 0,
        relics: hash_map! {Relics::RingOfTheSnake => 0},
        health: 70,
        max_health: 70,
        gold: 99,
        deck: vec![],
    };

    for _ in 0..5 {
        run.deck.push(CardInstance::new(Card::SilentStrike));
        run.deck.push(CardInstance::new(Card::SilentDefend));
    }
    run.deck.push(CardInstance::new(Card::Survivor));

    for card in [Card::Neutralize] {
        let mut samples = vec![];
        info!("Running simulations for {:?}", card);

        for i in 1..100 {
            let mut run = run.clone();
            run.deck.push(CardInstance::new(card));

            debug!("Starting simulation {i}");
            let run = run_simulation(run, &MapGenerator::generate());
            info!("Simulation {i} ended on floor {} with {} HP", run.floor, run.health);
            samples.push(run);
        }

        info!("Adding {:?}: expecting {:.2} health on floor {:.2} with {:.2} gold; {} deaths",
            card,
            samples.iter().map(|r| r.health).sum::<u32>() as f32 / samples.len() as f32,
            samples.iter().map(|r| r.floor).sum::<u32>() as f32 / samples.len() as f32,
            samples.iter().map(|r| r.gold).sum::<u32>() as f32 / samples.len() as f32,
            samples.iter().filter(|r| r.health == 0).count());
    }
}

fn run_simulation(mut run: Run, starting_room: &MapRoom) -> Run {
    let mut next_room = starting_room.up_nodes.get(0);

    while run.health > 0 && let Some(room) = next_room {
        run.floor += 1;
        
        debug!("Moving to floor {}: {:?}", run.floor, room.t);

        match &room.t {
            RoomType::Encounter(monsters, gold) | RoomType::Elite(monsters, gold) => {
                let mut encounter = Encounter::new(run);
                encounter.enemies.append(&mut monsters.iter().map(|m| Enemy::new(m.clone())).collect());

                run = run_encounter(encounter);
                if run.health > 0 {
                    run.gold += gold;
                } else {
                    break;
                }
            },
            RoomType::Ancient(_ancient) => {
                
            }
            RoomType::Treasure(_relic, gold) => {
                run.gold += gold;
                info!("Gained {} gold", gold);
            }
            RoomType::Rest => {
                let before = run.health;
                run.health = std::cmp::min(run.max_health, run.health + 15);
                info!("Healed {} HP", run.health - before);
            }
        };

        next_room = room.up_nodes.get(0);
    }

    run
}

fn run_encounter(mut encounter: Encounter) -> Run {

    // turn loop
    loop {
        encounter.begin_turn();
        let health = encounter.player.health;
        debug!("Starting turn {}", encounter.turn);
        debug!("Drew: {:?}", encounter.hand);

        // card loop
        loop {

            let next_enemy = encounter.enemies.iter().filter(|e| e.health > 0).nth(0);
            if let None = next_enemy {
                return encounter.end();
            }

            let next_enemy = next_enemy.unwrap();

            let mut mcts = Search::new();
            mcts.nodes.insert(0, ActionNode {
                id: 0,
                up: None,
                down: vec![],
                encounter: encounter.clone(),
                position: Search::get_position(&encounter),
                action: None,
                expanded: false,
                visited: false,
                evals: 0,
                wins: 0,
                uct: 0.
            });

            for i in 0..10_000 {
                mcts.next(0);
            }

            let mut immediate_children: Vec<(u32, f32)> = mcts.nodes[&0].down.iter().map(|n| (*n, mcts.nodes[&n].uct)).collect();
            immediate_children.sort_by(|a, b| b.1.total_cmp(&a.1));

            if immediate_children.len() == 0 {
                break
            }

            let best = immediate_children[0].0;

            match mcts.nodes[&best].action {
                Some(Action::PlaySelf(card)) => encounter.play(encounter.hand.iter().filter(|c| c.card == card).nth(0).unwrap().id, 0, vec![], &vec![]),
                Some(Action::PlayAgainst(card)) => encounter.play(encounter.hand.iter().filter(|c| c.card == card).nth(0).unwrap().id, next_enemy.id, vec![], &vec![]),
                Some(Action::NextTurn) => break,
                None => panic!("{}", mcts.nodes[&best])
            }
        }
        // let attack_damage = {
        //     let mut d = 0;
        //     for i in encounter.get_enemy_intent(next_enemy) {
        //         if let EncounterOp::AttackPlayer(_, dmg) = i {
        //             d += dmg;
        //         }
        //     }
        //     d
        // };

        // if attack_damage > 0 {
        //     respond_to_attack(&mut encounter, attack_damage);
        // } else {
        //     general_response(&mut encounter);
        // };

        encounter.yield_turn();
        encounter.end_turn();

        if encounter.player.health <= 0 || encounter.enemies.iter().map(|e| e.health).sum::<u32>() <= 0 {
            return encounter.end();
        }

        debug!("Took {} damage", health - encounter.player.health);
    }
}

fn respond_to_attack(encounter: &mut Encounter, damage: u32) {
    trace!("Need to block {} damage", damage);

    while encounter.player.energy > 0 && encounter.player.block < damage && encounter.hand.len() > 0 {
        if encounter.enemies.iter().map(|e| e.health).sum::<u32>() == 0 {
            break;
        }

        if let Some(survivor) = get_card!(Card::Survivor, encounter.hand) {

            let mut best_discard;
            if let Some(ricochet) = get_card!(Card::Ricochet, encounter.hand) {
                best_discard = vec![ricochet];
            } else {
                best_discard = encounter.hand.iter()
                    .filter(|i| i.id != survivor.id)
                    .collect();
            }

            best_discard.sort_by(|c1, c2| c2.cost.cmp(&c1.cost));

            if let Some(to_discard) = best_discard.first() {
                debug!("Playing Survivor and discarding {:?}", to_discard);
                trace(&encounter, &survivor.card, Some(&to_discard.card), None);
                encounter.play(survivor.id, 0, vec![to_discard.id], &mut vec![]);
            } else {
                debug!("Playing Survivor without discarding");
                trace(&encounter, &survivor.card, None, None);
                encounter.play(survivor.id, 0, vec![], &mut vec![]);
            }
        } else if let Some(defend) = get_card!(Card::SilentDefend, encounter.hand) {
            debug!("Playing a Defend");
            trace(&encounter, &defend.card, None, None);
            encounter.play(defend.id, 0, vec![], &mut vec![]);
        } else {
            break;
        }
    }

    general_response(encounter);
}

fn general_response(encounter: &mut Encounter) {
    while encounter.player.energy > 0 && encounter.hand.len() > 0 {
        let enemy= encounter.enemies.iter().filter(|e| e.health > 0).nth(0);
        if enemy.is_none() {
            break;
        }

        let blade_dance = get_card!(Card::BladeDance, encounter.hand);
        if let Some(card) = blade_dance {
            debug!("Playing {:?}", card);
            trace(&encounter, &card.card, None, None);
            encounter.play(card.id, 0, vec![], &vec![]);
            continue;
        }

        encounter.hand.sort_by(|c1, c2| c1.cost.cmp(&c2.cost));
        debug!("Playing {:?}", encounter.hand[0].card);
        match &CARDS[&encounter.hand[0].card].typ {
            CardType::Attack => {
                trace(&encounter, &encounter.hand[0].card, None, Some(enemy.unwrap()));
                encounter.play(encounter.hand[0].id, enemy.unwrap().id, vec![], &vec![]);
            },
            _ => {
                trace(&encounter, &encounter.hand[0].card, None, None);
                encounter.play(encounter.hand[0].id, 0, vec![], &vec![]);
            }
        };
    }
}

static LOG_FILE: LazyLock<Mutex<File>> = LazyLock::new(|| Mutex::new(OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open("trace.log")
        .unwrap()));


fn trace(encounter: &Encounter, card: &Card, other_card: Option<&Card>, enemy: Option<&Enemy>) {
}
//     let mut file = LOG_FILE.lock().unwrap();
//     let attack_damage;
//     if let [EncounterOp::AttackPlayer(_, dmg)] = encounter.get_enemy_intent(&encounter.enemies[0])[..] {
//         attack_damage = dmg;
//     } else {
//         attack_damage = 0;
//     }

//     let line = vec![
//         *card as u32,
//         encounter.hand.iter().filter(|c| c.card == Card::SilentDefend).count().try_into().unwrap(),
//         encounter.hand.iter().filter(|c| c.card == Card::SilentStrike).count().try_into().unwrap(),
//         encounter.hand.iter().filter(|c| c.card == Card::Neutralize).count().try_into().unwrap(),
//         encounter.hand.iter().filter(|c| c.card == Card::Survivor).count().try_into().unwrap(),
//         encounter.player.energy,
//         encounter.player.block,
//         encounter.enemies[0].health,
//         attack_damage
//     ]
//     .into_iter().join(",");

//     file.write(format!("{}\n", line).as_bytes()).expect("idk");
// }