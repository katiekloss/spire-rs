use rand::RngExt;

use crate::{cards::CustomCard, encounters::Encounter};

pub static RICOCHET: CustomCard = CustomCard {
    play: Some(|_, encounter| {
        let mut rng = rand::rng();
        let enemy_count = encounter.enemies.len();
        
        for _ in 0..4 {
            // TODO: make sure they're alive
            let target= &mut encounter.enemies[rng.random_range(0..enemy_count)];
            Encounter::resolve_attack(target, 3);
        }
    })
};