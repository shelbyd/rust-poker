use std::cmp::Ordering;

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

#[cfg(test)]
mod test {
    use super::Suit::{Diamond, Club, Heart, Spade};
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

    use super::Value::{Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King};
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
}
