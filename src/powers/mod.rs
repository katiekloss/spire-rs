use std::fmt::Debug;

use crate::{cards::CardInstance, core::Encounter};

pub mod defs;

type CardPlayed = fn(card: &CardInstance, encounter: &mut Encounter);
type TurnEnded = fn(card: &CardInstance, encounter: &mut Encounter);

pub struct PowerImpl {
    pub id: &'static str,
    pub card_played: Option<CardPlayed> = None,
    pub turn_ended: Option<TurnEnded> = None
}

impl Debug for PowerImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PowerImpl").field("id", &self.id).finish()
    }
}

impl PartialEq for PowerImpl {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
