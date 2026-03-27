use spire_rs::{Run, cards::{Card, CardInstance}, monsters::FuzzyWurmCrawler, services::EncounterManager};

fn main() {
    let mut run = Run {
        floor: 0,
        relics: vec![],
        health: 70,
        gold: 99
    };

    run.relics.push(spire_rs::relics::Relics::RingOfTheSnake);

    let mut encounter = EncounterManager::new(&run);
    encounter.enemies.push(Box::new(FuzzyWurmCrawler{}));

    assert_eq!(encounter.turn, 1);

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

        encounter.play(card);
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
