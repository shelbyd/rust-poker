mod card;
mod hand;
mod calculations;

#[cfg(not(test))]
fn main() {
    loop {
        let your_pocket = support::get_hand("Please enter your pocket: ");
        let community_cards = support::get_hand("Please enter the community cards: ");

        let (chance_of_winning, confident) = calculations::chance_of_winning(your_pocket, community_cards);

        match confident {
            true => println!("{:.2}% chance of winning", chance_of_winning * 100.0),
            false => println!("{:.2}% NOT CONFIDENT", chance_of_winning * 100.0),
        }
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
