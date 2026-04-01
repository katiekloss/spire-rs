use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use crate::{Keywords, encounters::Encounter, monsters::Enemy};

static CARDS: LazyLock<HashMap<Card, CardData>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Card::Neutralize, CardData{ cost: 0 });
    m.insert(Card::SilentStrike, CardData { cost: 1 });
    m.insert(Card::SilentDefend, CardData { cost: 1 });
    m.insert(Card::Survivor, CardData { cost: 1 });
    m
});

static CARD_IDS: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Card {
    SilentStrike,
    SilentDefend,
    Neutralize,
    Survivor
}

pub struct CardData {
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

pub enum PlayResult {
    NoOp,
    BlockableDamage(u32),
    GainBlock(u32)
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

    pub fn play(&mut self, _encounter: &Encounter) -> PlayResult {
        match self.card {
            Card::SilentDefend => {
                PlayResult::GainBlock(5)
            },
            _ => PlayResult::NoOp
        }
    }

    pub(crate) fn play_on(&self, _encounter: &Encounter, _target: &Enemy) -> PlayResult {
        match self.card {
            Card::SilentStrike => {
                PlayResult::BlockableDamage(6)
            },
            _ => PlayResult::NoOp
        }
    }
}