use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Eq, Ord, Hash)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade
}

impl Suit {
    fn _value(&self) -> u8 {
        match self {
            &Suit::Diamond => 0u8,
            &Suit::Club => 1u8,
            &Suit::Heart => 2u8,
            &Suit::Spade => 3u8
        }
    }
}

impl PartialEq for Suit {
    fn eq(&self, other: &Self) -> bool {
        self._value().eq(&other._value())
    }
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self._value().partial_cmp(&other._value())
    }
}

#[derive(Debug, Eq, Ord, Hash)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Value {
    fn _value(&self) -> u8 {
        match self {
            &Value::Two => 1u8,
            &Value::Three => 2u8,
            &Value::Four => 3u8,
            &Value::Five => 4u8,
            &Value::Six => 5u8,
            &Value::Seven => 6u8,
            &Value::Eight => 7u8,
            &Value::Nine => 8u8,
            &Value::Ten => 9u8,
            &Value::Jack => 10u8,
            &Value::Queen => 11u8,
            &Value::King => 12u8,
            &Value::Ace => 13u8,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self._value().eq(&other._value())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self._value().partial_cmp(&other._value())
    }
}

#[derive(Debug, Eq, Hash)]
pub struct Card {
    value: Value,
    suit: Suit,
}

impl FromStr for Card {
    type Err = CardParseErr;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut iter = s.chars();
        let value = match iter.next() {
            Some('A') => Some(Value::Ace),
            Some('2') => Some(Value::Two),
            Some('3') => Some(Value::Three),
            Some('4') => Some(Value::Four),
            Some('5') => Some(Value::Five),
            Some('6') => Some(Value::Six),
            Some('7') => Some(Value::Seven),
            Some('8') => Some(Value::Eight),
            Some('9') => Some(Value::Nine),
            Some('0') => Some(Value::Ten),
            Some('J') => Some(Value::Jack),
            Some('Q') => Some(Value::Queen),
            Some('K') => Some(Value::King),
            _ => None,
        };
        let suit = match iter.next() {
            Some('D') => Some(Suit::Diamond),
            Some('C') => Some(Suit::Club),
            Some('H') => Some(Suit::Heart),
            Some('S') => Some(Suit::Spade),
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

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) &&
        self.suit.eq(&other.suit)
    }
}

#[cfg(test)]
mod test {
    use super::Suit::*;
    #[test] fn suits_equal_themselves() {
        assert!(Diamond == Diamond);
        assert!(Club == Club);
        assert!(Heart == Heart);
        assert!(Spade == Spade);
    }

    #[test] fn suits_dont_equal_others() {
        assert!(Diamond != Club);
        assert!(Diamond != Heart);
        assert!(Diamond != Spade);

        assert!(Club != Diamond);
        assert!(Club != Heart);
        assert!(Club != Spade);

        assert!(Heart != Diamond);
        assert!(Heart != Club);
        assert!(Heart != Spade);

        assert!(Spade != Diamond);
        assert!(Spade != Club);
        assert!(Spade != Heart);
    }

    #[test] fn suits_are_ordered() {
        assert!(Diamond < Club);
        assert!(Diamond < Heart);
        assert!(Diamond < Spade);

        assert!(Club > Diamond);
        assert!(Club < Heart);
        assert!(Club < Spade);

        assert!(Heart > Diamond);
        assert!(Heart > Club);
        assert!(Heart < Spade);

        assert!(Spade > Diamond);
        assert!(Spade > Club);
        assert!(Spade > Heart);
    }

    use super::Value::*;
    #[test] fn values_equal_themselves() {
        assert!(Ace == Ace);
        assert!(Two == Two);
        assert!(Three == Three);
        assert!(Four == Four);
        assert!(Five == Five);
        assert!(Six == Six);
        assert!(Seven == Seven);
        assert!(Eight == Eight);
        assert!(Nine == Nine);
        assert!(Ten == Ten);
        assert!(Jack == Jack);
        assert!(Queen == Queen);
        assert!(King == King);
    }

    #[test] fn values_dont_equal_others() {
        assert!(Ace != Two);
        assert!(Two != Three);
        assert!(Three != Four);
        assert!(Four != Five);
        assert!(Five != Six);
        assert!(Six != Seven);
        assert!(Seven != Eight);
        assert!(Eight != Nine);
        assert!(Nine != Ten);
        assert!(Ten != Jack);
        assert!(Jack != Queen);
        assert!(Queen != King);
        assert!(King != Ace);
    }

    #[test] fn values_are_ordered() {
        assert!(Two < Three);
        assert!(Three < Four);
        assert!(Four < Five);
        assert!(Five < Six);
        assert!(Six < Seven);
        assert!(Seven < Eight);
        assert!(Eight < Nine);
        assert!(Nine < Ten);
        assert!(Ten < Jack);
        assert!(Jack < Queen);
        assert!(Queen < King);
        assert!(King < Ace);
        assert!(Two < Ace);
    }

    use super::Card;
    #[test] fn cards_have_value_and_suit() {
        let card = Card::new(Three, Heart);
        assert!(card.value == Three);
        assert!(card.suit == Heart);
    }
}
