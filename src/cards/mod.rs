use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use crate::{Keywords, encounters::Encounter};

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

impl CardInstance {
    pub fn new(card: Card) -> Self {
        Self {
            id: *CARD_IDS.lock().unwrap(),
            cost: CARDS[&card].cost,
            card,
            keywords: vec![]
        }
    }

    pub fn play(&mut self, encounter: &mut Encounter) {
        match self.card {
            Card::SilentDefend => {
                encounter.block += 5;
            },
            _ => {}
        }
    }
}