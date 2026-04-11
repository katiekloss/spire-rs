#![feature(default_field_values)]

use std::collections::HashMap;

use crate::{cards::CardInstance, powers::PowerImpl, relics::Relics};

pub mod cards;
pub mod relics;
pub mod monsters;
pub mod encounters;
pub mod map;
pub mod powers;

#[derive(Clone)]
pub struct Run {
    pub floor: u32,
    pub relics: HashMap<Relics, u32>,
    pub health: u32,
    pub gold: u32,
    pub max_health: u32,
    pub deck: Vec<CardInstance>,
}

/// A request from a relic, power, or card to perform a write operation against the encounter
pub enum EncounterOp {
    /// Direct unamped damage (.1) towards an enemy ID (.0)
    Damage(u32, u32),
    /// Set a relic counter to a specific value
    SetCounter(Relics, u32),
    SetHealth(u32),
    GainBlock(u32),
    /// Apply an effect to the player
    ApplySelf(Effect),
    /// Apply an effect to an enemy by ID
    ApplyTarget(u32, Effect)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Effect {
    Strength(u32),
    Weak(u32),
    Territorial(u32),
    Vulnerable(u32),
    Custom(&'static PowerImpl),
    Dexterity(u32)
}

pub trait Damageable {
    fn get_block(&self) -> u32;
    fn get_health(&self) -> u32;
    fn set_block(&mut self, block: u32);
    fn set_health(&mut self, health: u32);
}

pub trait Effectable {
    fn get_effects(&self) -> &Vec<Effect>;
}

pub trait Target : Damageable + Effectable {
    fn get_team(&self) -> Team;
    fn get_id(&self) -> u32;
}

pub enum Team {
    Friendly,
    Enemy
}

#[derive(Clone, PartialEq)]
pub enum Keywords {
    Eternal,
    Ethereal,
    Exhaust,
    Innate,
    Retain,
    Sly,
    Unplayable
}

/// Try to find a specific card in a pile of CardInstances (usually your hand)
/// ```
/// # use spire_rs::{get_card, cards::{CardInstance, library::Card}};
/// let hand = vec![CardInstance::new(Card::SilentDefend)];
/// let card = get_card!(Card::SilentDefend, hand).expect("aw beans, I don't have a defend!");
/// ```
#[macro_export]
macro_rules! get_card {
    ($card:path, $hand:expr) => {
        'get: {
            for c in &$hand {
                if let $card = c.card {
                    break 'get Some(c);
                }
            }
            None
        }
    }
}