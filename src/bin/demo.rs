use log::{debug, info, trace};
use spire_rs::{Run, cards::{CardInstance, CardType, library::{CARDS, Card}}, encounters::Encounter, get_card, map::{MapGenerator, MapRoom, RoomType}, monsters::{Enemy, Moves}, relics::Relics};
use std_logger::Config;

fn main() {
    Config::logfmt().init();

    let mut run = Run {
        floor: 0,
        relics: vec![Relics::RingOfTheSnake],
        health: 70,
        gold: 99,
        deck: vec![],
    };

    for _ in 0..5 {
        run.deck.push(CardInstance::new(Card::SilentStrike));
        run.deck.push(CardInstance::new(Card::SilentDefend));
    }
    run.deck.push(CardInstance::new(Card::Survivor));
    run.deck.push(CardInstance::new(Card::Neutralize));

    for card in [Card::Acrobatics, Card::BladeDance, Card::Ricochet] {
        let mut run = run.clone();
        run.deck.push(CardInstance::new(card));

        let mut samples = vec![];
        info!("Running simulations for {:?}", card);
        for i in 1..500_000 {
            debug!("Starting simulation {i}");
            let (health, floor) = run_simulation(&run, &MapGenerator::generate());
            debug!("Simulation {i} ended on floor {floor} with {health} HP");
            samples.push((health, floor));
        }

        info!("Adding {:?}: expecting {:.2} health on floor {:.2}; {} deaths",
            card,
            samples.iter().map(|s| s.0).sum::<u32>() as f32 / samples.len() as f32,
            samples.iter().map(|s| s.1).sum::<u32>() as f32 / samples.len() as f32,
            samples.iter().filter(|h| h.0 == 0).count());
    }
}

fn run_simulation(run: &Run, starting_room: &MapRoom) -> (u32, u32) {
    let mut run = run.clone();
    let mut next_room = starting_room.up_nodes.get(0);

    while run.health > 0 && let Some(room) = next_room {
        run.floor += 1;
        
        debug!("Moving to floor {}", run.floor);

        let mut encounter = Encounter::new(&run);
        match &room.t {
            RoomType::Encounter(monsters) => {
                encounter.enemies.append(&mut monsters.iter().map(|m| Enemy::new(m.clone())).collect());
            },
            _ => {
                todo!()
            }
        }

        run.health = run_encounter(encounter);
        if run.health == 0 {
            break;
        }

        next_room = room.up_nodes.get(0);
    }

    (run.health, run.floor)
}

fn run_encounter(mut encounter: Encounter) -> u32 {

    loop {
        encounter.begin_turn();
        let health = encounter.player.health;
        debug!("Starting turn {}", encounter.turn);
        debug!("Drew: {:?}", encounter.hand);

        match encounter.get_enemy_intent(&encounter.enemies[0]) {
            spire_rs::monsters::Moves::Attack(_) => respond_to_attack(&mut encounter),
            _ => general_response(&mut encounter),
        };

        encounter.yield_turn();
        encounter.end_turn();

        if encounter.player.health <= 0 || encounter.enemies[0].health <= 0 {
            return encounter.player.health;
        }

        debug!("Took {} damage", health - encounter.player.health);
    }
}

fn respond_to_attack(encounter: &mut Encounter) {
    let damage = match encounter.get_enemy_intent(&encounter.enemies[0]) {
        Moves::Attack(dmg) => dmg,
        _ => unreachable!()
    };

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
        let blade_dance = get_card!(Card::BladeDance, encounter.hand);
        if let Some(card) = blade_dance {
            debug!("Playing {:?}", card);
            encounter.play(card.id, 0, vec![], &vec![]);
            continue;
        }

        encounter.hand.sort_by(|c1, c2| c1.cost.cmp(&c2.cost));
        debug!("Playing {:?}", encounter.hand[0].card);
        match &CARDS[&encounter.hand[0].card].typ {
            CardType::Attack => encounter.play(encounter.hand[0].id, encounter.enemies[0].id, vec![], &vec![]),
            _ => encounter.play(encounter.hand[0].id, 0, vec![], &vec![]),
        };
    }
}