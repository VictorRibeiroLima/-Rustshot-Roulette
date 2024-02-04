use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    item::{list::ItemList, Item},
    shell::Shell,
    shotgun::Shotgun,
};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub max_health: u8,
    pub health: u8,
    pub items: ItemList,
    pub shotgun: Rc<RefCell<Shotgun>>,
    pub turn: bool,
    pub hand_cuffs: bool,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {} health:", self.name, self.health,)?;
        write!(f, "  Items:\n{}", self.items)?;
        Ok(())
    }
}

impl Player {
    pub fn new(name: String, shotgun: Rc<RefCell<Shotgun>>) -> Self {
        Self {
            name,
            health: 4,
            max_health: 4,
            items: ItemList::new(),
            shotgun,
            turn: false,
            hand_cuffs: false,
        }
    }

    pub fn new_round(&mut self, health: u8, turn: bool) {
        self.health = health;
        self.max_health = health;
        self.items = ItemList::new();
        self.turn = turn;
    }

    pub fn get_shot(&mut self, damage: u8) {
        let shotgun = self.shotgun.borrow();
        match self.health.checked_sub(damage) {
            Some(health) => self.health = health,
            None => self.health = 0,
        }
        if self.hand_cuffs {
            self.hand_cuffs = false;
        } else if !shotgun.empty() {
            self.turn = true;
        }
    }

    pub fn use_item(&mut self, item: Item, opponent: &mut Player) -> Result<Option<Shell>, ()> {
        let item = self.items.use_item(item);
        if let Some(item) = item {
            match item {
                Item::Beer => {
                    let mut shotgun = self.shotgun.borrow_mut();
                    let shell = shotgun.pump();
                    if shotgun.empty() {
                        self.turn = false;
                    }
                    return Ok(Some(shell));
                }
                Item::Saw => {
                    let mut shotgun = self.shotgun.borrow_mut();
                    shotgun.saw();
                    return Ok(None);
                }
                Item::MagnifyingGlass => {
                    let shotgun = self.shotgun.borrow();
                    let shell = shotgun.peak();
                    return Ok(Some(*shell));
                }
                Item::Cigarette => {
                    self.health += 1;
                    if self.health > self.max_health {
                        self.health = self.max_health;
                    }
                    return Ok(None);
                }
                Item::Handcuffs => {
                    opponent.hand_cuffs = true;
                    return Ok(None);
                }
            }
        }
        Err(())
    }

    pub fn shot_enemy(&mut self, opponent: &mut Player) -> Shell {
        let mut shotgun = self.shotgun.borrow_mut();
        let result = shotgun.fire();

        //Remove mutable borrow of shotgun so we can borrow it again
        drop(shotgun);
        let damage = result.damage;
        let shell = result.shell;
        let shotgun = self.shotgun.borrow();
        opponent.get_shot(damage);
        if opponent.turn || shotgun.empty() {
            self.turn = false;
        }
        shell
    }

    pub fn shot_self(&mut self) -> Shell {
        let mut shotgun = self.shotgun.borrow_mut();
        let result = shotgun.fire();
        let damage = result.damage;
        let shell = result.shell;
        match self.health.checked_sub(damage) {
            Some(damage) => self.health = damage,
            None => self.health = 0,
        };
        if self.health == 0 || shotgun.empty() {
            self.turn = false;
        }
        shell
    }
}
