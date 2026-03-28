use std::cmp::min;

use rand::seq::SliceRandom;

use crate::{Effect, Run, cards::CardInstance, monsters::Monster, relics::Relics};

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
    pub enemies: Vec<Box<dyn Monster>>,
    pub effects: Vec<Box<dyn Effect>>
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
        self.turn += 1;

        while let Some(card) = self.hand.pop() {
            self.discard_pile.push(card);
        }
    }

    pub fn play(&self, _card: &CardInstance) {

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
}

#[cfg(test)]
mod tests {
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