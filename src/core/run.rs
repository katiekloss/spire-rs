
use crate::{Run, RunOp, relics::{RELICS, Relics}};

impl Run {
    pub fn pickup_relic(&mut self, relic: Relics) {
        self.relics.insert(relic.clone(), 0);

        if let Some(picked_up) = &RELICS[&relic].picked_up {
            self.do_run_ops(picked_up(self));
        }
    }

    fn do_run_ops(&mut self, ops: Vec<RunOp>) {
        for op in ops {
            match op {
                RunOp::SetMaxHealth(hp) => {
                    let starting_health = self.max_health;
                    self.max_health = hp;
                    if starting_health < self.max_health {
                        self.health += self.max_health - starting_health;
                    }
                }
            }
        }
    }
}