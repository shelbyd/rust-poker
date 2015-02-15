mod card;
mod hand;

#[cfg(not(test))]
fn main() {
    use std::cmp::Ordering;

    loop {
        let your_hand = support::get_hand("Please enter your hand: ");
        let opponents_hand = support::get_hand("Please enter opponent's hand: ");
        let result = match your_hand.cmp(&opponents_hand) {
            Ordering::Equal => "It's a tie",
            Ordering::Greater => "You win!",
            Ordering::Less => "You lose",
        };
        println!("{}", result);
        println!("");
    }
}

#[cfg(not(test))]
mod support {
    use std::old_io;
    use hand::Hand;

    pub fn get_hand(prompt: &str) -> Hand {
		print!("{}", prompt);
        loop {
            let mut reader = old_io::stdin();
            match reader.read_line() {
                Ok(string) => {
                    match string.parse::<Hand>() {
                        Ok(hand) => return hand,
                        _ => {
                            could_not_interpret_input();
                            continue
                        },
                    }
                },
                Err(_) => {
                    could_not_interpret_input();
                    continue
                },
            }
        }
    }

    fn could_not_interpret_input() {
        println!("I don't recognize that as a poker hand...");
        println!("Sample input: AS 2H 0C QD 6H");
        print!("Please try again: ");
    }
}
