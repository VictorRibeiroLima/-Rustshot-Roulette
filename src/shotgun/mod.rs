use rand::seq::SliceRandom;

use crate::shell::{self, Shell};

#[derive(Debug, Clone)]
pub struct Shotgun {
    shells: Vec<Shell>,
    sawed: bool,
}

impl Shotgun {
    pub fn new(mut shells: Vec<Shell>) -> Self {
        let rng = &mut rand::thread_rng();
        shells.shuffle(rng);
        Self {
            shells,
            sawed: false,
        }
    }

    pub fn fire(&mut self) -> u8 {
        let shell = self.shells.pop().unwrap();
        match shell {
            Shell::Empty => 0,
            Shell::Loaded => {
                if self.sawed {
                    self.sawed = false;
                    2
                } else {
                    1
                }
            }
        }
    }

    pub fn pump(&mut self) -> Shell {
        let shell = self.shells.pop().unwrap();
        shell
    }

    pub fn peak(&self) -> &Shell {
        let shell = self.shells.last().unwrap();
        shell
    }

    pub fn saw(&mut self) {
        self.sawed = true;
    }
}
