use spire_rs::{Run, cards::{Card, CardInstance}, monsters::FuzzyWurmCrawler, encounters::Encounter};

fn main() {
    let mut run = Run {
        floor: 0,
        relics: vec![],
        health: 70,
        gold: 99,
        deck: vec![]
    };

    for _ in 0..5 {
        run.deck.push(CardInstance::new(Card::SilentStrike));
        run.deck.push(CardInstance::new(Card::SilentDefend));
    }
    run.deck.push(CardInstance::new(Card::Survivor));
    run.deck.push(CardInstance::new(Card::Neutralize));

    run.relics.push(spire_rs::relics::Relics::RingOfTheSnake);

    let mut encounter = Encounter::new(&run);
    encounter.enemies.push(Box::new(FuzzyWurmCrawler{}));

    {
        encounter.begin_turn();
        assert_eq!(encounter.hand.len(), 7); // ring of the snake
        assert_eq!(encounter.energy, 3);

        // macro this
        let card: &CardInstance = 'get: {
            for card in &encounter.hand {
                if let Card::SilentDefend = card.card {
                    break 'get card;
                }
            }

            panic!(); // lol uh oh
        };

        // mentioned elsewhere: this is gross, I need a more elegant solution to the borrow problem than this
        encounter.play_by_id(card.id);

        assert_eq!(encounter.hand.len(), 6);
        assert_eq!(encounter.energy, 2);
        assert_eq!(encounter.block, 5);

        encounter.end_turn();
    }

    assert_eq!(encounter.turn, 2);

    {
        encounter.begin_turn();
        assert_eq!(encounter.hand.len(), 5); // no more ring of the snake
        assert_eq!(encounter.energy, 3);
    }
}
