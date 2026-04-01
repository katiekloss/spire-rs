use spire_rs::{Run, cards::{Card, CardInstance}, encounters::Encounter, monsters::{Enemy, Monsters}};

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
    encounter.enemies.push(Enemy::new(Monsters::FuzzyWurmCrawler));

    {
        encounter.begin_turn();
        assert_eq!(encounter.turn, 1);
        println!("Turn {}: {:?}", encounter.turn, encounter.hand);

        assert_eq!(encounter.hand.len(), 7); // ring of the snake
        assert_eq!(encounter.player.energy, 3);

        // macro this
        let card: &CardInstance = 'get: {
            for card in &encounter.hand {
                if let Card::SilentDefend = card.card {
                    break 'get card;
                }
            }

            panic!(); // lol uh oh
        };

        // this is an example of the borrow issue, to get the card we have to borrow it from the encounter,
        // which immutably borrows the encounter so we can't mutably borrow it here. aaaaaaaaaaaaaaaaaaaaa
        encounter.play_by_id(card.id);

        assert_eq!(encounter.hand.len(), 6);
        assert_eq!(encounter.player.energy, 2);
        assert_eq!(encounter.player.block, 5);

        encounter.end_turn();
        encounter.commit_turn();
    }

    {
        encounter.begin_turn();
        assert_eq!(encounter.turn, 2);

        println!("Turn {}: {:?}", encounter.turn, encounter.hand);
        assert_eq!(encounter.hand.len(), 5); // no more ring of the snake
        assert_eq!(encounter.player.energy, 3);

        let card: &CardInstance = 'get: {
            for card in &encounter.hand {
                if let Card::SilentStrike = card.card {
                    break 'get card;
                }
            }

            panic!();
        };

        encounter.play_by_id_with_target(card.id, encounter.enemies[0].id);

        let card: &CardInstance = 'get: {
            for card in &encounter.hand {
                if let Card::SilentDefend = card.card {
                    break 'get card;
                }
            }

            panic!();
        };

        encounter.play_by_id(card.id);
        assert_eq!(encounter.player.block, 5);
        assert_eq!(encounter.player.energy, 1);
        assert_eq!(encounter.enemies[0].health, 49);

        encounter.end_turn();
        assert_eq!(encounter.player.block, 1);
        assert_eq!(encounter.player.health, 70);

        encounter.commit_turn();
    }

    {
        encounter.begin_turn();
        println!("Turn {}: {:?}", encounter.turn, encounter.hand);

        let card: &CardInstance = 'get: {
            for card in &encounter.hand {
                if let Card::SilentStrike = card.card {
                    break 'get card;
                }
            }

            panic!();
        };

        encounter.play_by_id_with_target(card.id, encounter.enemies[0].id);
        assert_eq!(encounter.enemies[0].health, 43);

        encounter.end_turn();
        assert_eq!(encounter.enemies[0].effects.len(), 1);

        encounter.commit_turn();
    }

    {
        encounter.begin_turn();
        println!("Turn {}: {:?}", encounter.turn, encounter.hand);

        let card: &CardInstance = 'get: {
            for card in &encounter.hand {
                if let Card::SilentDefend = card.card {
                    break 'get card;
                }
            }

            panic!();
        };

        encounter.play_by_id(card.id);

        let card: &CardInstance = 'get: {
            for card in &encounter.hand {
                if let Card::SilentDefend = card.card {
                    break 'get card;
                }
            }

            panic!("try again");
        };

        encounter.play_by_id(card.id);

        encounter.end_turn();
        assert_eq!(encounter.player.health, 69);

        encounter.commit_turn();
    }
}