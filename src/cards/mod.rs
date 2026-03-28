use std::{collections::HashMap, sync::LazyLock};

static CARDS: LazyLock<HashMap<Card, CardData>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Card::Neutralize, CardData{ cost: 0 });
    m.insert(Card::SilentStrike, CardData { cost: 1 });
    m.insert(Card::SilentDefend, CardData { cost: 1 });
    m.insert(Card::Survivor, CardData { cost: 1 });
    m
});

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Card {
    SilentStrike,
    SilentDefend,
    Neutralize,
    Survivor
}

pub struct CardData {
    pub cost: u8,
    // secondary_cost: u8 // regent
}

#[derive(Clone)]
pub struct CardInstance {
    pub card: Card,
    pub cost: u8,
    // secondary_cost: u8 // regent
}

impl CardInstance {
    pub fn new(card: Card) -> Self {
        Self {
            cost: CARDS[&card].cost,
            card
        }
    }
}