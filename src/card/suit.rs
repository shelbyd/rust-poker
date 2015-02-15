#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

impl Suit {
    pub fn from_char(char: char) -> Option<Suit> {
        match char {
            'D' | 'd' => Some(Suit::Diamond),
            'C' | 'c' => Some(Suit::Club),
            'H' | 'h' => Some(Suit::Heart),
            'S' | 's' => Some(Suit::Spade),
            _ => None,
        }
    }
}

mod tests {
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
}
