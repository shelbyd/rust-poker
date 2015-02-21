#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Copy, Clone)]
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

impl Value {
    pub fn from_char(char: char) -> Option<Value> {
        match char {
            'A' | 'a' => Some(Value::Ace),
            '2' => Some(Value::Two),
            '3' => Some(Value::Three),
            '4' => Some(Value::Four),
            '5' => Some(Value::Five),
            '6' => Some(Value::Six),
            '7' => Some(Value::Seven),
            '8' => Some(Value::Eight),
            '9' => Some(Value::Nine),
            '0' => Some(Value::Ten),
            'J' | 'j' => Some(Value::Jack),
            'Q' | 'q' => Some(Value::Queen),
            'K' | 'k' => Some(Value::King),
            _ => None
        }
    }
}


#[cfg(test)]
mod tests {
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
}
