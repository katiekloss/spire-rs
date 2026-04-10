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
    Afterimage,
    SuckerPunch,
    Adrenaline,
    Dash,
    LegSweep,
    LeadingStrike,
    Backstab,
    Deflect,
    Footwork,
    Reflex
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
    m.insert(Card::SuckerPunch, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::BlockableDamage(8), CardAction::Apply(Effect::Weak(1))], typ: CardType::Attack, custom: None});
    m.insert(Card::Adrenaline, CardData { cost: 0, keywords: vec![Keywords::Exhaust], actions: vec![CardAction::GainEnergy(1), CardAction::Draw(2)], typ: CardType::Skill, custom: None });
    m.insert(Card::Dash, CardData { cost: 2, keywords: vec![], actions: vec![CardAction::BlockableDamage(10), CardAction::GainBlock(10)], typ: CardType::Attack, custom: None });
    m.insert(Card::LegSweep, CardData { cost: 2, keywords: vec![], actions: vec![CardAction::Apply(Effect::Weak(2)), CardAction::GainBlock(11)], typ: CardType::Skill,..});
    m.insert(Card::LeadingStrike, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::BlockableDamage(7), CardAction::Materialize(Card::Shiv)], typ: CardType::Attack,.. });
    m.insert(Card::Backstab, CardData { cost: 0, keywords: vec![Keywords::Innate, Keywords::Exhaust], actions: vec![CardAction::BlockableDamage(11)], typ: CardType::Attack,..});
    m.insert(Card::Deflect, CardData { cost: 0, keywords: vec![], actions: vec![CardAction::GainBlock(4)], typ: CardType::Skill,..});
    m.insert(Card::Footwork, CardData { cost: 1, keywords: vec![], actions: vec![CardAction::Apply(Effect::Dexterity(2))], typ: CardType::Power,..});
    m.insert(Card::Reflex, CardData { cost: 3, keywords: vec![Keywords::Sly], actions: vec![CardAction::Draw(2)], typ: CardType::Skill,..});
    m
});