mod card;
mod hand;
mod calculations;

#[cfg(not(test))]
fn main() {
    loop {
        let your_pocket = support::get_hand("Please enter your pocket: ");
        let community_cards = support::get_hand("Please enter the community cards: ");
        let other_players = support::get_number("Please enter the number of opponents: ");

        let (chance_of_winning, confidence_interval) = calculations::chance_of_winning(your_pocket, community_cards, other_players);

        println!("{:.2}% ± {:.2}% chance of winning", chance_of_winning * 100.0, confidence_interval * 100.0);
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
                    match string.trim().parse::<Hand>() {
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

    pub fn get_number(prompt: &str) -> usize {
		print!("{}", prompt);
        loop {
            let mut reader = old_io::stdin();
            match reader.read_line() {
                Ok(string) => {
                    match string.trim().parse::<usize>() {
                        Ok(number) => return number,
                        _ => {
                            could_not_interpret_number_input();
                            continue
                        },
                    }
                },
                Err(_) => {
                    could_not_interpret_number_input();
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

    fn could_not_interpret_number_input() {
        println!("I don't recognize that as a number...");
        println!("Sample input: 4");
        print!("Please try again: ");
    }
}
