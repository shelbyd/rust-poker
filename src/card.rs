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
}
