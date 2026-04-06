#[cfg(test)]
mod card_tests {
    use spire_rs::{Run, cards::{Card, CardInstance}, encounters::Encounter, get_card, map::MapGenerator, monsters::{Enemy, Monsters}};

    fn start_run() -> Run {
        let run = Run {
            deck: vec![],
            floor: 1,
            gold: 0,
            health: 1,
            relics: vec![],
            current_room: MapGenerator::generate()
        };

        run
    }

    #[test]
    fn discarding_a_sly_card_plays_it_for_free() {
        let mut run = start_run();
        run.deck.push(CardInstance::new(Card::Survivor));
        run.deck.push(CardInstance::new(Card::FlickFlack));
        let mut encounter = Encounter::new(&run);
        encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));

        encounter.begin_turn();
        encounter.play(get_card!(Card::Survivor, encounter.hand).unwrap().id, 0, vec![get_card!(Card::FlickFlack, encounter.hand).unwrap().id], &mut vec![]);

        assert_eq!(encounter.hand.len(), 0);
        assert_eq!(encounter.discard_pile.len(), 2);
        assert_eq!(encounter.enemies[0].health, 48);

        assert_eq!(encounter.player.energy, 2);
    }

    #[test]
    fn playing_acrobatics_draws_up_to_three_cards() {
        let mut run = start_run();
        run.deck.append(&mut vec![
            CardInstance::new(Card::Acrobatics),
            CardInstance::new(Card::SilentStrike),
            CardInstance::new(Card::SilentStrike),
            CardInstance::new(Card::SilentStrike),
            CardInstance::new(Card::SilentStrike),
            CardInstance::new(Card::SilentDefend),
            CardInstance::new(Card::SilentDefend),
            CardInstance::new(Card::SilentDefend)
        ]);
        let mut encounter = Encounter::new(&run);

        let acrobatics = 'get: loop {
            encounter.begin_turn();
            if let Some(acrobatics) = get_card!(Card::Acrobatics, encounter.hand) {
                break 'get acrobatics;
            }
            encounter.yield_turn();
            encounter.end_turn();
        };

        encounter.play(acrobatics.id, 0, vec![], &mut vec![]);
        assert_eq!(encounter.hand.len(), 7);
    }
}