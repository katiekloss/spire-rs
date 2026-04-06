use std::cmp::min;

use rand::seq::SliceRandom;

use crate::{Damageable, Effect, Effectable, Keywords, Run, Target, Team, cards::{CARDS, CardAction, CardInstance}, monsters::{Enemy, Moves}, relics::Relics};

pub struct Player {
    pub energy: u32,
    pub block: u32,
    pub health: u32,
    pub effects: Vec<Effect>
}

impl Effectable for Player {
    fn get_effects(&self) -> &Vec<Effect> {
        &self.effects
    }
}

impl Target for Player {
    fn get_team(&self) -> crate::Team {
        Team::Friendly
    }

    fn get_id(&self) -> u32 {
        0
    }
}

pub struct Encounter<'a> {
    pub run: &'a Run,

    pub player: Player,
    pub draw_pile: Vec<CardInstance>,
    pub hand: Vec<CardInstance>,
    pub discard_pile: Vec<CardInstance>,
    pub exhaust_pile: Vec<CardInstance>,

    pub turn: u32,
    pub enemies: Vec<Enemy>,
}

impl<'a> Encounter<'a> {
    pub fn new(run: &'a Run) -> Self {
        let cards = run.deck.clone();            

        Self {
            run,
            turn: 0,
            enemies: vec![],
            draw_pile: vec![],
            hand: vec![],
            discard_pile: cards, // will be shuffled at the start of the first turn
            exhaust_pile: vec![],
            player: Player {
                health: run.health,
                effects: vec![],
                energy: 3,
                block: 0,
            }
    }}
    
    pub fn begin_turn(&mut self) {
        assert_eq!(self.player.block, 0, "Previous turn wasn't committed");

        self.turn += 1;

        self.refill_draw_pile();
        
        // put this somewhere elseeeeeeeeeeeeee
        let draw_size;
        if self.turn == 1 && self.run.relics.contains(&Relics::RingOfTheSnake) {
            draw_size = 7;
        } else {
            draw_size = 5;
        }

        self.draw(draw_size);
    }

    fn draw(&mut self, draw_size: usize) {
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

    pub fn yield_turn(&mut self) {

        self.resolve_enemies();
        self.discard_pile.append(&mut self.hand);
    }

    pub fn end_turn(&mut self) {
        // TODO: time-based effect ticks

        self.player.energy = 3;
        self.player.block = 0;
    }

    pub fn play(&mut self, card: u32, target_id: u32, other_cards: Vec<u32>, stack: &Vec<CardAction>) {
        let mut card = self.hand.swap_remove(self.find_card_in_hand(card));

        if card.keywords.contains(&Keywords::Sly) && stack.len() > 0 && let CardAction::Discard(_) = stack[stack.len() - 1] {
            // can be played for free
        } else {
            self.player.energy -= card.cost;
        }

        if let Some(custom) = CARDS[&card.card].custom && let Some(play) = custom.play {
            play(&mut card, self);
        }

        for action in CARDS[&card.card].actions.clone() {
            match action {
                CardAction::BlockableDamage(d) => {
                    if target_id == 0 {
                        todo!();
                    } else {
                        let enemy = self.enemies.iter_mut().filter(|e| e.id == target_id).nth(0).unwrap();
                        Self::resolve_attack(enemy, Self::query_attack_damage(&self.player, d));
                    }
                },
                CardAction::GainBlock(b) => self.player.block += b,
                CardAction::AffectSelf(x) => self.player.effects.push(x),
                CardAction::AffectAllOthers(x) => {
                    for enemy in self.enemies.iter_mut() {
                        enemy.effects.push(x.clone());
                    }
                },
                CardAction::Apply(x) => {
                    if target_id == 0 {
                        self.player.effects.push(x);
                    } else {
                        let enemy = self.enemies.iter_mut().filter(|e| e.id == target_id).nth(0).unwrap();
                        enemy.effects.push(x);
                    }
                },
                CardAction::Discard(_) => {
                    // if other_cards.len() != n.try_into().unwrap() {
                    //     panic!("Provided {} cards but only {} needs to be discarded", other_cards.len(), n);
                    // }
                    for id in &other_cards {
                        let i = self.find_card_in_hand(*id);
                        let card = &self.hand[i];

                        if card.keywords.contains(&Keywords::Sly) {
                            let mut stack = stack.clone();
                            stack.push(action);
                            self.play(card.id, target_id, vec![], &stack);
                            stack.pop();
                        } else {
                            self.discard_pile.push(self.hand.swap_remove(i));
                        }
                    }
                },
                CardAction::DamageAllOthers(d) => {
                    for enemy in self.enemies.iter_mut() {
                        Self::resolve_attack(enemy, d);
                    }
                },
                CardAction::Draw(n) => {
                    self.draw(n);
                },
                CardAction::Materialize(new_card) => {
                    self.hand.push(CardInstance::new(new_card));
                }
            }
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

    fn resolve_enemies(&mut self) {
        // will need to be mutable for thorns
        for enemy in self.enemies.iter_mut() {
            match &enemy.moves[enemy.move_idx] {
                Moves::Attack(dmg) => {
                    let dmg = Self::query_attack_damage(enemy, *dmg);
                    Self::resolve_attack(&mut self.player, dmg);
                },
                Moves::Buff(effect) => {
                    enemy.effects.push(effect.clone());
                },
                Moves::Debuff(effect) => {
                    self.player.effects.push(effect.clone());
                }
            }

            enemy.move_idx = (enemy.move_idx + 1) % enemy.moves.len();
        }
    }

    fn query_attack_damage<T: Effectable>(source: &T, base_damage: u32) -> u32 {
        let mut total_damage = base_damage;
        for effect in source.get_effects() {
            match effect {
                Effect::Strength(s) => total_damage += s,
                Effect::Weak(_) => total_damage = ((base_damage as f32) * 0.75).floor() as u32
            }
        }
        total_damage
    }

    pub fn resolve_attack<T: Damageable>(target: &mut T, damage: u32) {
        let block = target.get_block();

        // split damage into amount blocked vs amount that pierces block
        // there's an elegant way to do this, but i have a headache rn
        let (damage, blocked) = {
            if damage < block {
                (0, damage)
            } else {
                (damage - block, block)
            }
        };

        target.set_block(block - blocked);
        if damage > target.get_health() {
            target.set_health(0);
        } else {
            target.set_health(target.get_health() - damage);
        }
    }

    // I dislike this but the player needs a way to see the
    // effective attack damage without doing it themselves.
    pub fn get_enemy_intent(&self, enemy: &Enemy) -> Moves {
        match &enemy.moves[enemy.move_idx] {
            Moves::Attack(dmg) => {
                Moves::Attack(Self::query_attack_damage(enemy, *dmg))
            }
            x => x.clone()
        }
    }
}

impl Damageable for Player {
    fn get_block(&self) -> u32 {
        self.block
    }

    fn get_health(&self) -> u32 {
        self.health
    }

    fn set_block(&mut self, block: u32) {
        self.block = block;
    }

    fn set_health(&mut self, health: u32) {
        self.health = health;
    }
}

#[cfg(test)]
mod draw_tests {
    use crate::{Run, cards::CardInstance, encounters::Encounter, map::MapGenerator};

    fn start_run(cards: u32) -> Run {
        let mut run = Run {
            deck: vec![],
            floor: 1,
            gold: 0,
            health: 1,
            relics: vec![],
            current_room: MapGenerator::generate()
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
        encounter.yield_turn();
        encounter.end_turn();

        assert_eq!(1, encounter.draw_pile.len());
        assert_eq!(5, encounter.discard_pile.len());

        encounter.begin_turn();
        assert_eq!(5, encounter.hand.len());
        assert_eq!(0, encounter.discard_pile.len());
        assert_eq!(1, encounter.draw_pile.len());
    }
}