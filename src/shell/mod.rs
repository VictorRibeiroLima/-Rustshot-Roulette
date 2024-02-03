use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shell {
    Empty,
    Loaded,
}

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shell::Empty => write!(f, "Empty Shell"),
            Shell::Loaded => write!(f, "Loaded Shell"),
        }
    }
}

pub fn gen_shells() -> Vec<Shell> {
    let mut shells = vec![Shell::Empty, Shell::Loaded];
    let spins = rand::random::<u8>() % 6;
    println!("Spinning the chamber {} times", spins);
    for _ in 0..spins {
        let empty = rand::random::<u8>() % 2 == 0;
        if empty {
            shells.push(Shell::Empty);
        } else {
            shells.push(Shell::Loaded);
        }
    }
    shells
}
