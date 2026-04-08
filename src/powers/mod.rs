use crate::{cards::CardInstance, encounters::Encounter};

pub mod defs;

type CardPlayed = fn(card: &CardInstance, encounter: &mut Encounter);

pub struct PowerImpl {
    pub id: &'static str,
    pub card_played: Option<CardPlayed>
}

impl PartialEq for PowerImpl {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
