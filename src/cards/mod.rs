mod custom;

use std::{collections::HashMap, fmt::{Debug}, sync::{LazyLock, Mutex}};

use crate::{Effect, Keywords, cards::custom::*, encounters::Encounter};

pub static CARDS: LazyLock<HashMap<Card, CardData>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(Card::Neutralize, CardData{ cost: 0, keywords: vec![], actions: vec![CardAction::BlockableDamage(3), CardAction::Apply(Effect::Weak(1))], typ: CardType::Attack, custom: None });
    m.insert(Card::SilentStrike, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::BlockableDamage(6)], typ: CardType::Attack, custom: None  });
    m.insert(Card::SilentDefend, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::GainBlock(5)], typ: CardType::Skill, custom: None  });
    m.insert(Card::Survivor, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::Discard(1), CardAction::GainBlock(8)], typ: CardType::Skill, custom: None  });
    m.insert(Card::FlickFlack, CardData { cost: 1, keywords: vec![Keywords::Sly], actions: vec![CardAction::DamageAllOthers(7)], typ: CardType::Attack, custom: None  });
    m.insert(Card::Acrobatics, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::Draw(3), CardAction::Discard(1)], typ: CardType::Skill, custom: None  });
    m.insert(Card::BladeDance, CardData { cost: 1, keywords: vec![Keywords::Exhaust], actions: vec![CardAction::Materialize(Card::Shiv), CardAction::Materialize(Card::Shiv), CardAction::Materialize(Card::Shiv)], typ: CardType::Skill, custom: None });
    m.insert(Card::Shiv, CardData { cost: 0, keywords: vec![Keywords::Exhaust], actions: vec![CardAction::BlockableDamage(4)], typ: CardType::Attack, custom: None });
    m.insert(Card::Ricochet, CardData { cost: 2, keywords: vec![Keywords::Sly], actions: vec![], typ: CardType::Attack, custom: Some(&RICOCHET) });
    m
});

static CARD_IDS: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Card {
    SilentStrike,
    SilentDefend,
    Neutralize,
    Survivor,
    FlickFlack,
    Acrobatics,
    BladeDance,
    Shiv,
    Ricochet
}

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
