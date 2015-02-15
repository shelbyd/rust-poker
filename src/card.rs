use std::str::FromStr;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Copy)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Copy)]
pub enum Value {
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
    King,
    Ace,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy)]
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
