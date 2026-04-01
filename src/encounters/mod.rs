use std::cmp::min;

use rand::seq::SliceRandom;

use crate::{Effect, Keywords, Run, cards::{CardInstance, PlayResult}, monsters::Enemy, relics::Relics};

pub struct Encounter<'a> {
    pub run: &'a Run,

    pub energy: u32,
    pub block: u32,

    pub draw_pile: Vec<CardInstance>,
    pub hand: Vec<CardInstance>,
    pub discard_pile: Vec<CardInstance>,
    pub exhaust_pile: Vec<CardInstance>,

    pub turn: u32,
    pub health: u32,
    pub enemies: Vec<Enemy>,
    pub effects: Vec<Effect>
}

impl<'a> Encounter<'a> {
    pub fn new(run: &'a Run) -> Self {
        let cards = run.deck.clone();
        
        Self {
            run,
            turn: 0,
            health: run.health,
            enemies: vec![],
            effects: vec![],
            energy: 3,
            block: 0,
            draw_pile: vec![],
            hand: vec![],
            discard_pile: cards, // will be shuffled at the start of the first turn
            exhaust_pile: vec![]
    }}
    
    pub fn begin_turn(&mut self) {
        self.turn += 1;

        self.refill_draw_pile();
        
        // put this somewhere elseeeeeeeeeeeeee
        let draw_size;
        if self.turn == 1 && self.run.relics.contains(&Relics::RingOfTheSnake) {
            draw_size = 7;
        } else {
            draw_size = 5;
        }

        let hand_size = min(draw_size, self.draw_pile.len() + self.discard_pile.len());
        
        'hand: for _ in 0..hand_size {
            let card = match self.draw_pile.pop() {
                Some(c) => c,
                None => 'get: {
                    self.refill_draw_pile();
                    if let Some(c) = self.draw_pile.pop() {
                        break 'get c;
                    }
                    break 'hand;
                }
            };

            self.hand.push(card);
        }
    }

    pub fn end_turn(&mut self) {
        self.energy = 3;
        self.discard_pile.append(&mut self.hand);
    }

    pub fn play_by_id(&mut self, card: u32) {
        let i = self.find_card_in_hand(card);

        let mut card = self.hand.swap_remove(i);

        self.energy -= card.cost;

        let result = card.play(self);

        match result {
            PlayResult::GainBlock(b) => self.block += b,
            _ => {}
        }

        if card.keywords.contains(&Keywords::Exhaust) {
            self.exhaust_pile.push(card);
        } else {
            self.discard_pile.push(card);
        }
    }

    pub fn play_by_id_with_target(&mut self, card: u32, target: u32) {
        let i = self.find_card_in_hand(card);
        let e = 'get: {
            for e in 0..self.enemies.len() {
                if self.enemies[e].id == target {
                    break 'get e;
                }
            }
            panic!("Can't find enemy");
        };

        let card = self.hand.swap_remove(i);
        self.energy -= card.cost;

        let result = card.play_on(self,&self.enemies[e]);

        match result {
            PlayResult::BlockableDamage(d) => {
                let (blocked, damage) = Self::split_damage(d, &self.enemies[e]);
                self.enemies[e].block -= blocked;
                self.enemies[e].health -= damage;
            },
            PlayResult::GainBlock(b) => {
                self.enemies[e].block += b;
            },
            _ => {}
        }

        if card.keywords.contains(&Keywords::Exhaust) {
            self.exhaust_pile.push(card);
        } else {
            self.discard_pile.push(card);
        }
    }

    #[inline(always)]
    fn find_card_in_hand(&self, card: u32) -> usize {
        for i in 0..self.hand.len() {
            if self.hand[i].id == card {
                return i;
            }
        }
        panic!("Can't find card");
    }

    #[inline]
    fn refill_draw_pile(&mut self) {
        if !self.draw_pile.is_empty() {
            return;
        }

        while self.discard_pile.len() > 0 {
            self.draw_pile.push(self.discard_pile.pop().unwrap());
        }
        self.draw_pile.shuffle(&mut rand::rng());
    }

    fn handle_result(&mut self, card: &CardInstance, results: PlayResult, target: Option<Enemy>) {

    }

    /// Calculate damage absorbed by an enemy's block and piercing damage that lowers their health
    fn split_damage(mut damage: u32, target: &Enemy) -> (u32, u32) {
        if damage < target.block {
            return (damage, 0);
        }

        damage -= target.block;
        return (target.block, damage);
    }
}

#[cfg(test)]
mod draw_tests {
    use crate::{Run, cards::CardInstance, encounters::Encounter};

    fn start_run(cards: u32) -> Run {
        let mut run = Run {
            deck: vec![],
            floor: 1,
            gold: 0,
            health: 1,
            relics: vec![]
        };

        for _ in 0..cards {
            run.deck.push(CardInstance::new(crate::cards::Card::SilentDefend));
        }

        run
    }

    #[test]
    fn regular_draw() {
        let run = start_run(6);
        let mut encounter = Encounter::new(&run);
        encounter.begin_turn();

        assert_eq!(1, encounter.draw_pile.len());
        assert_eq!(5, encounter.hand.len());
    }

    #[test]
    fn draw_with_shuffle() {
        let run = start_run(6);
        let mut encounter = Encounter::new(&run);
        encounter.begin_turn();
        encounter.end_turn();

        assert_eq!(1, encounter.draw_pile.len());
        assert_eq!(5, encounter.discard_pile.len());

        encounter.begin_turn();
        assert_eq!(5, encounter.hand.len());
        assert_eq!(0, encounter.discard_pile.len());
        assert_eq!(1, encounter.draw_pile.len());
    }
}