use crate::{item::Item, player::Player, shell::Shell};

#[derive(Debug, Clone, Copy)]
pub enum Predict {
    Known(Shell),
    Unknown(Shell),
}

fn predict_shell(player: &Player) -> Predict {
    let shotgun = player.shotgun.borrow();
    let (empty, loaded) = shotgun.shell_count();
    if empty == 0 {
        return Predict::Known(Shell::Loaded);
    }
    if loaded == 0 {
        return Predict::Known(Shell::Empty);
    }
    let loaded = loaded as f64;
    let empty = empty as f64;
    let total = loaded + empty;
    let loaded_percentage = loaded / total;
    let empty_percentage = empty / total;
    if loaded_percentage > empty_percentage {
        return Predict::Unknown(Shell::Loaded);
    } else {
        return Predict::Unknown(Shell::Empty);
    };
}

pub fn take_action(player: &mut Player, opponent: &mut Player) {
    if !player.turn {
        return;
    }

    try_use_cigarette(player, opponent);

    let predict = predict_shell(opponent);

    let predict = match predict {
        Predict::Known(_) => predict,
        Predict::Unknown(unknown) => try_use_mag_glass(player, opponent, unknown),
    };

    let predict = try_use_beer(player, opponent, predict);
    if !player.turn {
        return;
    }

    let shot_enemy = match predict {
        Predict::Known(shell) => {
            if shell == Shell::Empty {
                false
            } else {
                true
            }
        }
        Predict::Unknown(shell) => {
            if shell == Shell::Empty {
                false
            } else {
                true
            }
        }
    };

    if shot_enemy {
        try_use_handcuffs(player, opponent);
        try_use_sawn(player, opponent);
        let shell = player.shot_enemy(opponent);
        println!("{} shot {} with a {}", player.name, opponent.name, shell);
    } else {
        let shell = player.shot_self(opponent);
        println!("{} shot themselves with a {}", player.name, shell);
    }
}

fn try_use_mag_glass(player: &mut Player, opponent: &mut Player, unknown: Shell) -> Predict {
    let items = &player.items;
    let mag_glass = Item::MagnifyingGlass;
    let has_magnifying_glass = items.has_item(mag_glass);
    if has_magnifying_glass {
        let shell = player
            .use_item(mag_glass, opponent)
            .expect("Player has magnifying glass")
            .expect("Shell inside shotgun");
        Predict::Known(shell)
    } else {
        Predict::Unknown(unknown)
    }
}

fn try_use_beer(player: &mut Player, opponent: &mut Player, predict: Predict) -> Predict {
    if !player.turn {
        return predict;
    }
    match predict {
        Predict::Known(_) => {
            return predict;
        }
        Predict::Unknown(_) => {}
    };

    let items = &player.items;
    let beer = Item::Beer;
    let has_beer = items.has_item(beer);
    if has_beer {
        let result = player.use_item(beer, opponent).expect("Player has beer");
        if let Some(shell) = result {
            println!("{} used a beer and pumped a {}", player.name, shell);
        }

        let predict = predict_shell(&player);
        return try_use_beer(player, opponent, predict);
    }
    return predict;
}

fn try_use_cigarette(player: &mut Player, opponent: &mut Player) {
    if player.max_health == player.health {
        return;
    }
    let items = &player.items;
    let cigarette = Item::Cigarette;
    let has_cigarette = items.has_item(cigarette);
    if has_cigarette {
        player
            .use_item(cigarette, opponent)
            .expect("Player has cigarette");

        try_use_cigarette(player, opponent);
    }
}

fn try_use_sawn(player: &mut Player, opponent: &mut Player) {
    let shotgun = player.shotgun.borrow();
    if shotgun.is_sawed() {
        return;
    }
    drop(shotgun);
    let items = &player.items;
    let saw = Item::Saw;
    let has_saw = items.has_item(saw);
    if has_saw {
        player.use_item(saw, opponent).expect("Player has saw");
    }
}

fn try_use_handcuffs(player: &mut Player, opponent: &mut Player) {
    if opponent.hand_cuffs {
        return;
    }
    let items = &player.items;
    let handcuffs = Item::Handcuffs;
    let has_handcuffs = items.has_item(handcuffs);
    if has_handcuffs {
        player
            .use_item(handcuffs, opponent)
            .expect("Player has handcuffs");
    }
}
