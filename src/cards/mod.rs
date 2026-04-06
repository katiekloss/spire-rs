mod custom;
pub mod library;

use std::{fmt::{Debug}, sync::{LazyLock, Mutex}};

use crate::{Effect, Keywords, cards::library::{CARDS, Card}, encounters::Encounter};

static CARD_IDS: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));


pub enum CardType {
    Skill,
    Power,
    Attack,
    Status
}

pub struct CardData {
    pub typ: CardType,
    pub actions: Vec<CardAction>,
    pub keywords: Vec<Keywords>,
    pub cost: u32,
    pub custom: Option<&'static CustomCard>
    // secondary_cost: u8 // regent
}

#[derive(Clone)]
pub struct CardInstance {
    pub id: u32, // TODO: this is gross ew yucky (but avoids the borrow checker)
    pub card: Card,
    pub cost: u32,
    // secondary_cost: u8 // regent
    pub keywords: Vec<Keywords>,
    pub custom: Option<&'static CustomCard>
}

impl Debug for CardInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardInstance").field("id", &self.id).field("card", &self.card).finish()
    }
}

#[derive(Clone, Copy)]
pub enum CardAction {
    BlockableDamage(u32),
    Draw(usize),
    DamageAllOthers(u32),
    Discard(u32),
    GainBlock(u32),
    AffectSelf(Effect),
    AffectAllOthers(Effect),
    Apply(Effect),
    Materialize(Card)
}

impl CardInstance {
    pub fn new(card: Card) -> Self {
        Self {
            id: { let mut i = CARD_IDS.lock().unwrap(); *i += 1; *i },
            cost: CARDS[&card].cost,
            keywords: CARDS[&card].keywords.clone(),
            custom: CARDS[&card].custom,
            card,
        }
    }
}

type PlayHandler = fn(&mut CardInstance, &mut Encounter);

pub struct CustomCard {
    pub play: Option<PlayHandler> // = None // enable the default_field_values feature when there are more fields and it becomes annoying
}
