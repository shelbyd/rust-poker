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
            Some(char) => Value::from_char(char),
            _ => None,
        };
        let suit = match iter.next() {
            Some(char) => Suit::from_char(char),
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
