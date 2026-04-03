use std::{collections::HashMap, fmt::{Debug}, sync::{LazyLock, Mutex}};

use crate::{Effect, Keywords, encounters::Encounter, monsters::Enemy};

static CARDS: LazyLock<HashMap<Card, CardData>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Card::Neutralize, CardData{ cost: 0, results: CardResults::Targeted(vec![TargetedPlayResult::BlockableDamage(3), TargetedPlayResult::Debuff(Effect::Weak(1))]) });
    m.insert(Card::SilentStrike, CardData { cost: 1, results: CardResults::Targeted(vec![TargetedPlayResult::BlockableDamage(6)]) });
    m.insert(Card::SilentDefend, CardData { cost: 1, results: CardResults::PlaysOnSelf(vec![SelfPlayResult::GainBlock(5)]) });
    m.insert(Card::Survivor, CardData { cost: 1, results: CardResults::PlaysOnSelf(vec![SelfPlayResult::Discard(1), SelfPlayResult::GainBlock(8)]) });
    m
});

static CARD_IDS: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Card {
    SilentStrike,
    SilentDefend,
    Neutralize,
    Survivor
}

#[derive(Clone)]
pub enum CardResults {
    PlaysOnSelf(Vec<SelfPlayResult>),
    Targeted(Vec<TargetedPlayResult>)
}

pub struct CardData {
    pub results: CardResults,
    pub cost: u32,
    // secondary_cost: u8 // regent
}

#[derive(Clone)]
pub struct CardInstance {
    pub id: u32, // TODO: this is gross ew yucky (but avoids the borrow checker)
    pub card: Card,
    pub cost: u32,
    // secondary_cost: u8 // regent
    pub keywords: Vec<Keywords>
}

impl Debug for CardInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardInstance").field("id", &self.id).field("card", &self.card).finish()
    }
}

#[derive(Clone, Copy)]
pub enum SelfPlayResult {
    Discard(u32),
    GainBlock(u32),
    AffectSelf(Effect),
    AffectAllOthers(Effect)
}

#[derive(Clone, Copy)]
pub enum TargetedPlayResult {
    BlockableDamage(u32),
    Buff(Effect),
    Debuff(Effect)
}

impl CardInstance {
    pub fn new(card: Card) -> Self {
        Self {
            id: { let mut i = CARD_IDS.lock().unwrap(); *i += 1; *i },
            cost: CARDS[&card].cost,
            card,
            keywords: vec![]
        }
    }

    // keeping these separate because I feel like they'll eventually be more complicated
    pub fn play(&mut self, _encounter: &Encounter) -> Vec<SelfPlayResult> {
        match &CARDS[&self.card].results {
            CardResults::PlaysOnSelf(results) => results.clone(),
            _ => panic!()
        }
    }

    pub fn play_on(&self, _encounter: &Encounter, _target: &Enemy) -> Vec<TargetedPlayResult> {
        match &CARDS[&self.card].results {
            CardResults::Targeted(results) => results.clone(),
            _ => panic!()
        }
    }
}