use std::collections::HashMap;

use rand::{RngExt, rngs::ThreadRng};
use spire_rs::{EncounterOp, Run, cards::{CardInstance, library::Card}, core::Encounter, monsters::{Enemy, Monsters}};

static EXPLORE_RATE: f64 = 1.;
const EXPLORE_DECAY: f64 = 0.99999975;

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
    let mut rng = rand::rng();
    let mut mem = Memory {

    };

    loop {
        let encounter = start_encounter();
        
        while encounter.player.health > 0 && encounter.enemies[0].health > 0 {
           
        }
    }

    Ok(())
}

pub fn play_from(rng: &mut ThreadRng, mut encounter: Encounter, position: &Position) {
    let actions = get_all_actions(&encounter);
    if rng.random_range(0. .. 1.) < EXPLORE_RATE {
        let our_action = &actions[rng.random_range(0..actions.len())];
    }
}

fn get_position(encounter: &mut Encounter) -> Position {
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

pub struct Memory {
    pub states: HashMap<Position, Action>
}

pub enum Action {
    PlaySelf(Card),
    PlayAgainst(Card)
}

pub struct Position {
    pub hand: Vec<Card>,
    pub incoming_damage: u32,
    pub block: u32
}