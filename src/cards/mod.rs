mod custom;
pub mod library;

use std::{fmt::{Debug}, sync::{LazyLock, Mutex}};

use crate::{Effect, Keywords, cards::library::{CARDS, Card}, encounters::Encounter};

static CARD_IDS: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

#[derive(Clone, PartialEq)]
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
    pub custom: Option<&'static CustomCard> = None
    // secondary_cost: u8 // regent
}

#[derive(Clone)]
pub struct CardInstance {
    pub id: u32, // TODO: this is gross ew yucky (but avoids the borrow checker)
    pub card: Card,
    pub cost: u32,
    // secondary_cost: u8 // regent
    pub keywords: Vec<Keywords>,
    pub custom: Option<&'static CustomCard>,
    pub typ: CardType
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
    Materialize(Card),
    GainEnergy(u32)
}

impl CardInstance {
    pub fn new(card: Card) -> Self {
        let def = &CARDS[&card];
        Self {
            id: { let mut i = CARD_IDS.lock().unwrap(); *i += 1; *i },
            cost: def.cost,
            keywords: def.keywords.clone(),
            custom: def.custom,
            typ: def.typ.clone(),
            card,
        }
    }
}

type PlayHandler = fn(card: &mut CardInstance, encounter: &mut Encounter);

pub struct CustomCard {
    /// Called whenever an instance of this card is played
    pub play: Option<PlayHandler>,
}
