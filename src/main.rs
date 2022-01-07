use rand::prelude::*;
use std::io;
use std::process::Command;

pub const MIN_NUMBER: i16 = 1;
pub const MAX_NUMBER: i16 = 1000;

fn win() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("YoU WiN!");
    println!("All of code this game is on my GitHub\nhttps://github.com/SolindekDev/guessing-game-rust\n\nWait 3 seconds...");
    let mut child = Command::new("sleep").arg("3").spawn().unwrap();
    let _result = child.wait().unwrap();
}

fn game_loop(random: i16) {
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line!");

    let choice_i16: i16 = choice.trim().parse().expect("Please type a number!");

    if choice_i16 > random {
        println!("You number is bigger than guess number!");
        game_loop(random);
    } else if choice_i16 < random {
        println!("You number is smaller than guess number!");
        game_loop(random);
    } else if choice_i16 == random {
        win();
    }
}

fn main() {
    println!("-----------------------------");
    println!("     Guessing Number Rust    ");
    println!("-----------------------------");
    println!("Guess a random number from 0 to 100...");

    let mut rng = thread_rng();
    let random: i16 = rng.gen_range(MIN_NUMBER, MAX_NUMBER);

    game_loop(random);
}
