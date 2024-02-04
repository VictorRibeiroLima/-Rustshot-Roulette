use rand::seq::SliceRandom;

use crate::shell::Shell;

#[derive(Debug, Clone)]
pub struct Shotgun {
    shells: Vec<Shell>,
    sawed: bool,
}
pub struct FireResult {
    pub damage: u8,
    pub shell: Shell,
}

impl Shotgun {
    pub fn is_sawed(&self) -> bool {
        self.sawed
    }

    pub fn shell_count(&self) -> (usize, usize) {
        let mut empty = 0;
        let mut loaded = 0;
        for shell in &self.shells {
            match shell {
                Shell::Empty => empty += 1,
                Shell::Loaded => loaded += 1,
            }
        }
        (empty, loaded)
    }

    pub fn empty(&self) -> bool {
        self.shells.is_empty()
    }

    pub fn new(mut shells: Vec<Shell>) -> Self {
        let rng = &mut rand::thread_rng();
        shells.shuffle(rng);
        Self {
            shells,
            sawed: false,
        }
    }

    pub fn fire(&mut self) -> FireResult {
        let shell = self.shells.pop().unwrap();
        match shell {
            Shell::Empty => FireResult { damage: 0, shell },
            Shell::Loaded => {
                if self.sawed {
                    self.sawed = false;
                    FireResult { damage: 2, shell }
                } else {
                    FireResult { damage: 1, shell }
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
