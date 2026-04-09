#![cfg(test)]
use spire_rs::{Run, cards::{ CardInstance, library::Card}, encounters::Encounter, get_card, monsters::{Enemy, Monsters}};

#[test]
fn hello_world() {
    let mut run = Run {
        floor: 0,
        relics: vec![],
        health: 70,
        max_health: 70,
        gold: 99,
        deck: vec![],
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

        let card = get_card!(Card::SilentDefend, encounter.hand).unwrap();
        encounter.play(card.id, 0, vec![], &mut vec![]);

        assert_eq!(encounter.hand.len(), 6);
        assert_eq!(encounter.player.energy, 2);
        assert_eq!(encounter.player.block, 5);

        encounter.yield_turn();
        encounter.end_turn();
    }

    {
        encounter.begin_turn();
        assert_eq!(encounter.turn, 2);

        println!("Turn {}: {:?}", encounter.turn, encounter.hand);
        assert_eq!(encounter.hand.len(), 5); // no more ring of the snake
        assert_eq!(encounter.player.energy, 3);

        let card = get_card!(Card::SilentStrike, encounter.hand).unwrap();
        encounter.play(card.id, encounter.enemies[0].id, vec![], &vec![]);

        let card = get_card!(Card::SilentDefend, encounter.hand).unwrap();
        encounter.play(card.id, 0, vec![], &mut vec![]);

        assert_eq!(encounter.player.block, 5);
        assert_eq!(encounter.player.energy, 1);

        encounter.yield_turn();
        assert_eq!(encounter.player.block, 1);
        assert_eq!(encounter.player.health, 70);

        encounter.end_turn();
    }

    {
        encounter.begin_turn();
        println!("Turn {}: {:?}", encounter.turn, encounter.hand);

        let card = get_card!(Card::SilentStrike, encounter.hand).unwrap();
        encounter.play(card.id, encounter.enemies[0].id, vec![], &vec![]);

        encounter.yield_turn();
        assert_eq!(encounter.enemies[0].effects.len(), 1);

        encounter.end_turn();
    }

    {
        encounter.begin_turn();
        println!("Turn {}: {:?}", encounter.turn, encounter.hand);

        if let Some(survivor) = get_card!(Card::Survivor, encounter.hand) {
            let discard = encounter.hand.iter().filter(|i| i.id != survivor.id).nth(0).unwrap();
            println!("Playing Survivor, discarding {:?}", discard);
            encounter.play(survivor.id, 0, vec![discard.id], &mut vec![]);
        } else {
            println!("Playing two Defends");
            let card = get_card!(Card::SilentDefend, encounter.hand).unwrap();
            encounter.play(card.id, 0, vec![], &mut vec![]);

            let card = get_card!(Card::SilentDefend, encounter.hand).expect("try again");
            encounter.play(card.id, 0, vec![], &mut vec![]);
        }

        assert_eq!(3, encounter.hand.len());

        encounter.yield_turn();

        encounter.end_turn();
    }
}