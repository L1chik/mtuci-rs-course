// Guess coin game

use std::io::stdin;

use rand::prelude::*;
fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let coin: String = rng.gen_range(0u8..=1u8).to_string();
        let mut guess = String::from("");
        println!("guess side (0 or 1)");
        stdin().read_line(&mut guess).unwrap();
        guess = guess.trim().to_string();
        if guess.as_str() != "0" && guess.as_str() != "1" {
            continue;
        } else if guess == coin {
            println!("you won!");
        } else {
            println!("you lose :(\nIt was {}", coin);
        }
    }
}
