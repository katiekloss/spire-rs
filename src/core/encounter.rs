use std::cmp::min;

use rand::seq::SliceRandom;

use crate::{Damageable, Effect, Effectable, EncounterOp, Keywords, Run, Target, Team, cards::{CardAction, CardInstance, CardType, library::CARDS}, core::{Encounter, Player}, monsters::{Enemy, EnemyMoves, Moves}, relics::{PlayTarget, RELICS, Relics}};

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

impl<'a> Encounter<'a> {
    pub fn new(run: &'a mut Run) -> Self {
        let cards = run.deck.clone();            

        Self {
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
            },
            run,
            this_turn: vec![]
    }}
    
    pub fn begin_turn(&mut self) {
        assert_eq!(self.player.block, 0, "Previous turn wasn't committed");

        self.this_turn.clear();
        self.turn += 1;

        if self.turn == 1 {
            let mut ops = vec![];
            for relic in self.run.relics.keys() {
                if let Some(combat_started) = RELICS[&relic].combat_started {
                    ops.append(&mut combat_started(self));
                }
            }
            self.do_encounter_ops(ops);
        }

        let mut ops = vec![];
        for relic in self.run.relics.keys() {
            if let Some(turn_started) = RELICS[&relic].turn_started {
                ops.append(&mut turn_started(self));
            }
        }
        self.do_encounter_ops(ops);

        // Tick effects
        let mut effects = vec![];
        while let Some(effect) = self.player.effects.pop() {
            match effect {
                Effect::Vulnerable(v) if v > 1 => effects.push(Effect::Vulnerable(v - 1)),
                Effect::Vulnerable(_) => {},
                _ => effects.push(effect),
            }
        }
        self.player.effects = effects;

        for enemy in &mut self.enemies {
            let mut effects = vec![];
            while let Some(effect) = enemy.effects.pop() {
                match effect {
                    Effect::Vulnerable(v) if v > 1 => effects.push(Effect::Vulnerable(v - 1)),
                    Effect::Vulnerable(_) => {},
                    _ => effects.push(effect),
                }
            }
            enemy.effects = effects;
        }

        self.refill_draw_pile();
        
        // put this somewhere elseeeeeeeeeeeeee
        let draw_size;
        if self.turn == 1 && self.run.relics.contains_key(&Relics::RingOfTheSnake) {
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
        for enemy in self.enemies.iter_mut() {
            let effects = enemy.effects.clone(); // boooooooooooooooooooooo do better
            for effect in effects {
                match effect {
                    Effect::Territorial(str) => enemy.effects.push(Effect::Strength(str)),
                    _ => {}
                }
            }
        }

        self.player.energy = 3;
        self.player.block = 0;
    }

    pub fn play(&mut self, card: u32, target_id: u32, other_cards: Vec<u32>, stack: &Vec<CardAction>) {
        let mut card = self.hand.swap_remove(self.find_card_in_hand(card));

        self.this_turn.push(EncounterOp::Play(card.card));

        // maybe macro this
        let card_played = 'get: {
            for effect in &self.player.effects {
                if let Effect::Custom(handler) = effect && let Some(card_played) = handler.card_played {
                    break 'get Some(card_played);
                }
            }
            None
        };

        let mut ops = vec![];
        for relic in self.run.relics.keys() {
            if let Some(played) = &RELICS[&relic].card_played {
                ops.append(&mut played(&card, PlayTarget::None, self));
            }
        }
        self.do_encounter_ops(ops);

        if let Some(handler) = card_played {
            handler(&card, self);
        }

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
                        Self::resolve_attack(enemy, Self::query_attack_damage(&self.player, enemy, d));
                    }
                },
                CardAction::GainBlock(b) => self.player.block += b, // TODO: Dexterity
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
                        let mut further = vec![];
                        let card = &self.hand[i];

                        if card.keywords.contains(&Keywords::Sly) {
                            let mut stack = stack.clone();
                            stack.push(action);
                            self.play(card.id, target_id, vec![], &stack);
                            stack.pop();
                        } else {
                            for relic in &self.run.relics { // booooooooooooooo
                                if let Some(discarded) = RELICS[&relic.0].card_discarded {
                                    further.append(&mut discarded(card, self));
                                }
                            }

                            self.discard_pile.push(self.hand.swap_remove(i));
                        }

                        self.do_encounter_ops(further);

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
                },
                CardAction::GainEnergy(e) => {
                    self.player.energy += e;
                }
            }
        }

        if let CardType::Power = card.typ {
            // Powers disappear entirely, instead of discarding or exhausting
        }
        else if card.keywords.contains(&Keywords::Exhaust) {
            self.exhaust_pile.push(card);
        } else {
            self.discard_pile.push(card);
        }
    }

    fn do_encounter_ops(&mut self, ops: Vec<EncounterOp>) {
        for op in ops {
            match op {
                EncounterOp::SetCounter(relic, new_value) => {
                    self.run.relics.insert(relic, new_value);
                },
                EncounterOp::Damage(target_id, damage) => {
                    self.basic_attack(target_id, damage);
                }
                EncounterOp::SetHealth(hp) => {
                    self.player.health = hp;
                }
                EncounterOp::GainBlock(b) => {
                    self.player.block += b;
                }
                EncounterOp::ApplySelf(fx) => {
                    self.player.effects.push(fx);
                },
                EncounterOp::ApplyTarget(enemy_id, fx) => {
                    let enemy = self.enemies.iter_mut().filter(|e| e.id == enemy_id).nth(0).unwrap();
                    enemy.effects.push(fx);
                },
                EncounterOp::AttackPlayer(enemy_id, dmg) => {
                    let enemy = self.enemies.iter().filter(|e| e.id == enemy_id).nth(0).unwrap();
                    let dmg = Self::query_attack_damage(enemy, &self.player, dmg);
                    Self::resolve_attack(&mut self.player, dmg);
                },
                EncounterOp::Materialize(card) => {
                    self.draw_pile.push(CardInstance::new(card));
                },
                EncounterOp::Play(_) => {}
            }
            
            self.this_turn.push(op);
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

        // TODO: Innate
        while self.discard_pile.len() > 0 {
            self.draw_pile.push(self.discard_pile.pop().unwrap());
        }
        self.draw_pile.shuffle(&mut rand::rng());
    }

    fn resolve_enemies(&mut self) {
        let enemy_ops: Vec<Vec<EncounterOp>> = self.enemies.iter().filter(|e| e.health > 0)
            .map(|e| {
                match &e.moves {
                    EnemyMoves::Static(moves) => {
                        let mut ops = vec![];
                        for mv in moves[e.move_idx].iter() {
                            ops.push(match mv {
                                Moves::Attack(dmg) => {
                                    EncounterOp::AttackPlayer(e.id, *dmg)
                                },
                                Moves::Buff(effect) => {
                                    EncounterOp::ApplyTarget(e.id, *effect)
                                },
                                Moves::Debuff(effect) => {
                                    EncounterOp::ApplySelf(*effect)
                                },
                                Moves::StatusCard(card) => {
                                    EncounterOp::Materialize(*card)
                                }
                            });
                        }
                        ops
                    },
                    EnemyMoves::Custom(handler) => {
                        handler(self)
                    }
                }
            })
            .collect();

        for set in enemy_ops {
            self.do_encounter_ops(set);
        }

        for enemy in self.enemies.iter_mut().filter(|e| e.health > 0) {
            if let EnemyMoves::Static(moves) = &enemy.moves {
                enemy.move_idx = (enemy.move_idx + 1) % moves.len();
            }
        }
    }

    fn query_attack_damage<TS: Effectable, TT: Effectable>(source: &TS, target: &TT, base_damage: u32) -> u32 {
        let mut total_damage = base_damage;
        for effect in source.get_effects() {
            match effect {
                Effect::Strength(s) => total_damage += s,
                Effect::Weak(_) => total_damage = ((base_damage as f32) * 0.75).floor() as u32,
                _ => {}
            }
        }

        for effect in target.get_effects() {
            match effect {
                Effect::Vulnerable(_) => total_damage = (total_damage as f32 * 1.5).floor() as u32,
                _ => {}
            }
        }
        total_damage
    }

    /// Attacks an enemy without applying damage amplification effects
    pub fn basic_attack(&mut self, target_id: u32, damage: u32) {
        let enemy = 'get: {
            for enemy in &mut self.enemies {
                if enemy.id == target_id {
                    break 'get enemy;
                }
            }
            panic!();
        };
        Self::resolve_attack(enemy, damage);
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
    pub fn get_enemy_intent(&self, enemy: &Enemy) -> Vec<EncounterOp> {
        match &enemy.moves {
            EnemyMoves::Static(moves) => {
                moves[enemy.move_idx].iter()
                    .map(|mv| {
                        match mv {
                            Moves::Attack(dmg) => {
                                EncounterOp::AttackPlayer(enemy.id, Self::query_attack_damage(enemy, &self.player, *dmg))
                            },
                            _ => EncounterOp::Play(crate::cards::library::Card::Acrobatics)
                        }
                    })
                    .collect()
            },
            EnemyMoves::Custom(handler) => handler(self)
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
    use std::collections::HashMap;

    use crate::{Run, cards::{CardInstance, library::Card}, core::encounter::Encounter};

    fn start_run(cards: u32) -> Run {
        let mut run = Run {
            deck: vec![],
            floor: 1,
            gold: 0,
            health: 1,
            max_health: 1,
            relics: HashMap::new(),
        };

        for _ in 0..cards {
            run.deck.push(CardInstance::new(Card::SilentDefend));
        }

        run
    }

    #[test]
    fn regular_draw() {
        let mut run = start_run(6);
        let mut encounter = Encounter::new(&mut run);
        encounter.begin_turn();

        assert_eq!(1, encounter.draw_pile.len());
        assert_eq!(5, encounter.hand.len());
    }

    #[test]
    fn draw_with_shuffle() {
        let mut run = start_run(6);
        let mut encounter = Encounter::new(&mut run);
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