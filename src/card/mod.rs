use std::str::FromStr;

mod value;
mod suit;

pub use self::value::Value;
pub use self::suit::Suit;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Ord, PartialOrd, Clone)]
pub struct Card {
    value: Value,
    suit: Suit,
}

impl FromStr for Card {
    type Err = CardParseErr;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut iter = s.chars();
        let value = match iter.next() {
            Some('A') | Some('a') => Some(Value::Ace),
            Some('2') => Some(Value::Two),
            Some('3') => Some(Value::Three),
            Some('4') => Some(Value::Four),
            Some('5') => Some(Value::Five),
            Some('6') => Some(Value::Six),
            Some('7') => Some(Value::Seven),
            Some('8') => Some(Value::Eight),
            Some('9') => Some(Value::Nine),
            Some('0') => Some(Value::Ten),
            Some('J') | Some('j') => Some(Value::Jack),
            Some('Q') | Some('q') => Some(Value::Queen),
            Some('K') | Some('k') => Some(Value::King),
            _ => None,
        };
        let suit = match iter.next() {
            Some('D') | Some('d') => Some(Suit::Diamond),
            Some('C') | Some('c') => Some(Suit::Club),
            Some('H') | Some('h') => Some(Suit::Heart),
            Some('S') | Some('s') => Some(Suit::Spade),
            _ => None,
        };
        match (value, suit) {
            (Some(value), Some(suit)) => Ok(Card::new(value, suit)),
            _ => Err(CardParseErr::Err),
        }
    }
}

pub enum CardParseErr {
    Err
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Card {
        Card { value: value, suit: suit }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }
}

#[cfg(test)]
mod test {
    use super::{Card, Value, Suit};
    #[test] fn cards_have_value_and_suit() {
        let card = Card::new(Value::Three, Suit::Heart);
        assert!(card.value == Value::Three);
        assert!(card.suit == Suit::Heart);
    }
}
