use std::{
    cell::RefCell,
    io::{self, Write},
};

use crate::player::Player;

mod item;
mod player;
mod shell;
mod shotgun;

fn main() {
    let shells = shell::gen_shells();
    let shotgun = shotgun::Shotgun::new(shells);
    let max_health = rand::random::<u8>() % 2 + 2;
    let shotgun = RefCell::new(shotgun);
    let mut player1 = Player::new("Player 1".to_string(), shotgun.clone());
    let mut player2 = Player::new("Player 2".to_string(), shotgun.clone());

    player1.new_round(max_health, true);
    player2.new_round(max_health, false);

    println!("{:?}", shotgun);
    println!("{}", player1);
    println!("{}", player2);

    loop {
        let input = get_input();
        let input = input.trim();
        match input {
            "1" => {
                let item = item::Item::Beer;
                let result = player1.use_item(item, &mut player2);
                match result {
                    Ok(shell) => {
                        if let Some(shell) = shell {
                            println!("Player 1 used a beer and got a {:?}", shell);
                        } else {
                            println!("Player 1 used a beer but there are no more left");
                        }
                    }
                    Err(_) => {
                        println!("Player 1 does not have any beers left");
                    }
                }
            }
            "2" => {
                let item = item::Item::Saw;
                let result = player1.use_item(item, &mut player2);
                match result {
                    Ok(_) => {
                        println!("Player 1 used a saw");
                    }
                    Err(_) => {
                        println!("Player 1 does not have a saw");
                    }
                }
            }
            "3" => {
                let item = item::Item::MagnifyingGlass;
                let result = player1.use_item(item, &mut player2);
                match result {
                    Ok(shell) => {
                        if let Some(shell) = shell {
                            println!("Player 1 used a magnifying glass and saw a {:?}", shell);
                        } else {
                            println!(
                                "Player 1 used a magnifying glass but there are no more shells"
                            );
                        }
                    }
                    Err(_) => {
                        println!("Player 1 does not have a magnifying glass");
                    }
                }
            }
            "4" => {
                let item = item::Item::Cigarette;
                let result = player1.use_item(item, &mut player2);
                match result {
                    Ok(_) => {
                        println!("Player 1 used a cigarette");
                    }
                    Err(_) => {
                        println!("Player 1 does not have a cigarette");
                    }
                }
            }
            "5" => {
                let item = item::Item::Handcuffs;
                let result = player1.use_item(item, &mut player2);
                match result {
                    Ok(_) => {
                        println!("Player 1 used handcuffs");
                    }
                    Err(_) => {
                        println!("Player 1 does not have handcuffs");
                    }
                }
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Unknown command");
            }
        }

        if !player1.turn {
            break;
        }
    }
}

fn get_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
