use std::{collections::HashMap, fmt::Display};

use crate::item::Item;

#[derive(Debug)]
pub struct ItemList {
    pub items: HashMap<u8, u8>,
}

impl Display for ItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printable = String::new();
        for (item, count) in &self.items {
            let name = Item::get_name(*item);
            printable.push_str(&format!("   {}: {}\n", name, count));
        }
        write!(f, "{}", printable)
    }
}

impl ItemList {
    pub fn new() -> Self {
        let mut spins = rand::random::<u8>() % 3;
        spins += 1;
        let mut items = HashMap::new();
        for _ in 0..spins {
            let item = rand::random::<u8>() % 5;
            let count = items.entry(item).or_insert(0);
            *count += 1;
        }
        Self { items }
    }

    pub fn use_item(&mut self, item: Item) -> Option<Item> {
        let index = item as u8;
        if let Some(count) = self.items.get_mut(&index) {
            if *count > 0 {
                *count -= 1;
                return Some(item);
            }
            return None;
        }
        None
    }
}
