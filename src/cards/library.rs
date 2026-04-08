use std::{collections::HashMap, sync::LazyLock};

use crate::{Effect, Keywords, cards::{CardAction, CardData, CardType, custom::*}, powers::defs};

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
    Ricochet,
    Slimed,
    DaggerSpray,
    CloakAndDagger,
    Afterimage
}

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
    m.insert(Card::Slimed, CardData { cost: 1, keywords: vec![Keywords::Exhaust], actions: vec![CardAction::Draw(1)], typ: CardType::Status, custom: None });
    m.insert(Card::DaggerSpray, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::DamageAllOthers(4), CardAction::DamageAllOthers(4)], typ: CardType::Attack, custom: None });
    m.insert(Card::CloakAndDagger, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::GainBlock(6), CardAction::Materialize(Card::Shiv)], typ: CardType::Skill, custom: None });
    m.insert(Card::Afterimage, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::Apply(Effect::Custom(&defs::AFTERIMAGE))], typ: CardType::Power, custom: None });
    m
});