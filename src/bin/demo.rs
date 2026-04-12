#![feature(hash_map_macro)]

use std::hash_map;

use log::{debug, info, trace};
use spire_rs::{EncounterOp, Run, cards::{CardInstance, CardType, library::{CARDS, Card}}, core::Encounter, get_card, map::{MapGenerator, MapRoom, RoomType}, monsters::{Enemy, Monsters}, relics::Relics};
use std_logger::Config;

fn main() {
    Config::logfmt().init();

    let mut run = Run {
        floor: 0,
        relics: hash_map! {Relics::RingOfTheSnake => 0, Relics::Anchor => 0},
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
    run.deck.push(CardInstance::new(Card::Neutralize));

    for card in [Card::CloakAndDagger, Card::BladeDance, Card::Ricochet, Card::DaggerSpray] {
        let mut samples = vec![];
        info!("Running simulations for {:?}", card);

        for i in 1..100_000 {
            let mut run = run.clone();
            run.deck.push(CardInstance::new(card));

            debug!("Starting simulation {i}");
            let (health, floor) = run_simulation(&mut run, &MapGenerator::generate());
            debug!("Simulation {i} ended on floor {floor} with {health} HP");
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

fn run_simulation(run: &mut Run, starting_room: &MapRoom) -> (u32, u32) {
    let mut next_room = starting_room.up_nodes.get(0);

    while run.health > 0 && let Some(room) = next_room {
        run.floor += 1;
        
        debug!("Moving to floor {}", run.floor);

        match &room.t {
            RoomType::Encounter(monsters, gold) | RoomType::Elite(monsters, gold) => {
                if !run_encounter_room(monsters, run, *gold) {
                    break;
                }
            },
            RoomType::Ancient(_ancient) => {
                
            }
            RoomType::Treasure(_relic, gold) => {
                run.gold += gold;
            }
            RoomType::Rest => {
                run.health += 15;
            }
        };

        next_room = room.up_nodes.get(0);
    }

    (run.health, run.floor)
}

/// Runs an encounter and returns whether the player is still alive afterward
fn run_encounter_room(monsters: &[Monsters], run: &mut Run, reward: u32) -> bool {
    let mut encounter = Encounter::new(run);
    encounter.enemies.append(&mut monsters.iter().map(|m| Enemy::new(m.clone())).collect());

    run.health = run_encounter(encounter);
    if run.health > 0 {
        run.gold += reward;
        return true;
    }

    false
}

fn run_encounter(mut encounter: Encounter) -> u32 {

    loop {
        encounter.begin_turn();
        let health = encounter.player.health;
        debug!("Starting turn {}", encounter.turn);
        debug!("Drew: {:?}", encounter.hand);

        let next_enemy = encounter.enemies.iter().filter(|e| e.health > 0).nth(0);
        if let None = next_enemy {
            return encounter.player.health;
        }
        let next_enemy = next_enemy.unwrap();
        let attack_damage = {
            let mut d = 0;
            for i in encounter.get_enemy_intent(next_enemy) {
                if let EncounterOp::AttackPlayer(_, dmg) = i {
                    d += dmg;
                }
            }
            d
        };

        if attack_damage > 0 {
            respond_to_attack(&mut encounter, attack_damage);
        } else {
            general_response(&mut encounter);
        };

        encounter.yield_turn();
        encounter.end_turn();

        if encounter.player.health <= 0 || encounter.enemies.iter().map(|e| e.health).sum::<u32>() <= 0 {
            return encounter.player.health;
        }

        debug!("Took {} damage", health - encounter.player.health);
    }
}

fn respond_to_attack(encounter: &mut Encounter, damage: u32) {
    trace!("Need to block {} damage", damage);

    while encounter.player.energy > 0 && encounter.player.block < damage && encounter.hand.len() > 0 {
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
                encounter.play(survivor.id, 0, vec![to_discard.id], &mut vec![]);
            } else {
                debug!("Playing Survivor without discarding");
                encounter.play(survivor.id, 0, vec![], &mut vec![]);
            }
        } else if let Some(defend) = get_card!(Card::SilentDefend, encounter.hand) {
            debug!("Playing a Defend");
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
            encounter.play(card.id, 0, vec![], &vec![]);
            continue;
        }

        encounter.hand.sort_by(|c1, c2| c1.cost.cmp(&c2.cost));
        debug!("Playing {:?}", encounter.hand[0].card);
        match &CARDS[&encounter.hand[0].card].typ {
            CardType::Attack => encounter.play(encounter.hand[0].id, enemy.unwrap().id, vec![], &vec![]),
            _ => encounter.play(encounter.hand[0].id, 0, vec![], &vec![]),
        };
    }
}