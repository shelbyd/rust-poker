mod card;
mod hand;

use std::old_io;
use hand::Hand;
use std::cmp::Ordering;

#[cfg(not(test))]
fn main() {
    loop {
        print!("Please enter your hand: ");
        let your_hand = get_hand();
        print!("Please enter opponent's hand: ");
        let opponents_hand = get_hand();
        let result = match your_hand.cmp(&opponents_hand) {
            Ordering::Equal => "It's a tie",
            Ordering::Greater => "You win!",
            Ordering::Less => "You lose",
        };
        println!("{}", result);
        println!("");
    }
}

fn get_hand() -> Hand {
    loop {
        let mut reader = old_io::stdin();
        match reader.read_line() {
            Ok(string) => {
                match string.parse::<Hand>() {
                    Ok(hand) => return hand,
                    _ => {
                        println!("Could not read that");
                        continue
                    },
                }
            },
            Err(_) => {
                println!("Could not read that");
                continue
            },
        }
    }
}
