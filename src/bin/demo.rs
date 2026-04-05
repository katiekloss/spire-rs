use log::{debug, info, trace};
use spire_rs::{Run, cards::{Card, CardInstance}, encounters::Encounter, get_card, map::MapGenerator, monsters::{Enemy, Monsters, Moves}, relics::Relics};
use std_logger::Config;

fn main() {
    Config::logfmt().init();

    let mut run = Run {
        floor: 0,
        relics: vec![Relics::RingOfTheSnake],
        health: 70,
        gold: 99,
        deck: vec![],
        current_room: MapGenerator::generate(),
    };

    for _ in 0..5 {
        run.deck.push(CardInstance::new(Card::SilentStrike));
        run.deck.push(CardInstance::new(Card::SilentDefend));
    }
    run.deck.push(CardInstance::new(Card::Survivor));
    run.deck.push(CardInstance::new(Card::Neutralize));

    let mut ending_health = vec![];
    for i in 1..100 {
        let health = run_encounter(&run);
        info!("Simulation {i} ended with {health} HP");
        ending_health.push(health);
    }

    info!("Expecting to walk away with {:.2} health", ending_health.iter().sum::<u32>() as f32 / ending_health.len() as f32);
    info!("I died {} times", ending_health.iter().filter(|h| **h == 0).count());
}

fn run_encounter(run: &Run) -> u32 {
    let mut encounter = Encounter::new(&run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));

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
            // try not to discard Neutralize
            let mut best_discard: Vec<&CardInstance> = encounter.hand.iter()
                .filter(|i| i.id != survivor.id)
                .collect();
            best_discard.sort_by(|c1, c2| c2.cost.cmp(&c1.cost));

            if let Some(to_discard) = best_discard.first() {
                debug!("Playing Survivor and discarding {:?}", to_discard);
                encounter.play_by_id(survivor.id, vec![to_discard.id]);
            } else {
                debug!("Playing Survivor without discarding");
                encounter.play_by_id(survivor.id, vec![]);
            }
        } else if let Some(defend) = get_card!(Card::SilentDefend, encounter.hand) {
            debug!("Playing a Defend");
            encounter.play_by_id(defend.id, vec![]);
        } else {
            break;
        }
    }

    general_response(encounter);
}

fn general_response(encounter: &mut Encounter) {
    while encounter.player.energy > 0 && encounter.hand.len() > 0 {
        if let Some(attack) = get_card!(Card::SilentStrike, encounter.hand) {
            debug!("Playing an Attack");
            encounter.play_by_id_with_target(attack.id, encounter.enemies[0].id);
        } else if let Some(neutralize) = get_card!(Card::Neutralize, encounter.hand) {
            debug!("Playing Neutralize");
            encounter.play_by_id_with_target(neutralize.id, encounter.enemies[0].id);
        } else {
            trace!("Nothing else to do");
            break;
        }
    }
}