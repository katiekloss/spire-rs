use rand::RngExt;

use crate::{cards::CustomCard, encounters::Encounter, monsters::Enemy};

pub static RICOCHET: CustomCard = CustomCard {
    play: Some(|_, encounter| {
        let mut rng = rand::rng();
        
        for _ in 0..4 {
            let mut alive: Vec<&mut Enemy> = encounter.enemies.iter_mut().filter(|e| e.health > 0).collect();
            let n = alive.len();
            if n == 0 {
                break;
            }
            
            Encounter::resolve_attack(alive[rng.random_range(..n)], 3);
        }
    }),
};
