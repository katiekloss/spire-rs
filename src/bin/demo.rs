use spire_rs::{Run, cards::{Card, CardInstance}, encounters::Encounter, get_card, map::MapGenerator, monsters::{Enemy, Monsters, Moves}, relics::Relics};

fn main() {
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

    let mut encounter = Encounter::new(&run);
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));

    loop {
        encounter.begin_turn();
        let health = encounter.player.health;
        println!("Starting turn {}", encounter.turn);
        println!("Drew: {:?}", encounter.hand);

        match encounter.get_enemy_intent(&encounter.enemies[0]) {
            spire_rs::monsters::Moves::Attack(_) => respond_to_attack(&mut encounter),
            _ => general_response(&mut encounter),
        };

        encounter.yield_turn();
        encounter.end_turn();

        println!("Took {} damage", health - encounter.player.health);

        if encounter.player.health <= 0 {
            println!("I died");
            break;
        } else if encounter.enemies[0].health <= 0 {
            println!("I won with {} HP remaining", encounter.player.health);
            break;
        }
    }
}

fn respond_to_attack(encounter: &mut Encounter) {
    let damage = match encounter.get_enemy_intent(&encounter.enemies[0]) {
        Moves::Attack(dmg) => dmg,
        _ => unreachable!()
    };

    println!("Need to block {} damage", damage);

    while encounter.player.energy > 0 && encounter.player.block < damage && encounter.hand.len() > 0 {
        if let Some(survivor) = get_card!(Card::Survivor, encounter.hand) {
            if let Some(to_discard) = encounter.hand.iter().filter(|i| i.id != survivor.id).nth(0) {
                println!("Playing Survivor and discarding {:?}", to_discard);
                encounter.play_by_id(survivor.id, vec![to_discard.id]);
            } else {
                println!("Playing Survivor without discarding");
                encounter.play_by_id(survivor.id, vec![]);
            }
        } else if let Some(defend) = get_card!(Card::SilentDefend, encounter.hand) {
            println!("Playing a Defend");
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
            println!("Playing an Attack");
            encounter.play_by_id_with_target(attack.id, encounter.enemies[0].id);
        } else if let Some(neutralize) = get_card!(Card::Neutralize, encounter.hand) {
            println!("Playing Neutralize");
            encounter.play_by_id_with_target(neutralize.id, encounter.enemies[0].id);
        } else {
            println!("Nothing else to do");
            break;
        }
    }
}