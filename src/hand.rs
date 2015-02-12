use card::{Card, Value};
use std::cmp::Ordering;
use std::collections::HashMap;

enum HandRank {
    HighCard,
    Pair,
}

impl HandRank {
    fn _value(&self) -> u8 {
        match self {
            &HandRank::HighCard => 0,
            &HandRank::Pair => 1,
        }
    }
}

impl PartialEq for HandRank {
    fn eq(&self, other: &Self) -> bool {
        self._value().eq(&other._value())
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self._value().partial_cmp(&other._value())
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Hand {
        Hand { cards: cards }
    }

    fn categorize(&self) -> HandRank {
        if self.cards.iter().any(|card| self.cards.iter().filter(|inner_card| card.value() == inner_card.value()).count() > 1) {
            HandRank::Pair
        } else {
            HandRank::HighCard
        }
    }

    fn most_common_value(&self) -> Option<&Value> {
        let mapped = self.cards.iter()
                                 .map(|card| card.value())
                                 .fold(HashMap::new(), |mut map, value| {
                                     let new_value = match map.get(&value) {
                                         Some(n) => n + 1,
                                         None => 1
                                     };
                                     map.insert(value, new_value);
                                     map
                                 });
        let default = &0;
        let max_count = match mapped.iter().map(|(_, count)| count).max() {
            Some(v) => v,
            None => default,
        };
        match mapped
            .iter()
            .filter(|&(&value, count)| count == max_count)
            .max_by(|&(&value, _)| value) {
                Some((&v, _)) => Some(v),
                None => None
            }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.categorize().eq(&other.categorize()) {
            true => self.most_common_value().eq(&other.most_common_value()),
            false => false
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.categorize().partial_cmp(&other.categorize()) {
            Some(Ordering::Equal) => self.most_common_value().partial_cmp(&other.most_common_value()),
            other => other
        }
    }
}

#[cfg(test)]
mod test {
    use card::Value::*;
    use card::Suit::*;
    use card::Card;
    use super::Hand;

    #[test] fn high_card_equals_high_card() {
        let high_card = Hand::new(vec![
            Card::new(Five, Diamond),
            Card::new(Six, Heart),
            Card::new(Seven, Heart),
            Card::new(Ace, Spade),
            Card::new(Jack, Diamond),
        ]);
        let other_high_card = Hand::new(vec![
            Card::new(Five, Heart),
            Card::new(Six, Club),
            Card::new(Seven, Club),
            Card::new(Ace, Spade),
            Card::new(Jack, Diamond),
        ]);

        assert!(high_card == other_high_card);
    }

    #[test] fn high_card_beats_a_lower_high_card() {
        let ace_high_card = Hand::new(vec![
            Card::new(Ace, Spade),
            Card::new(Six, Heart),
            Card::new(Seven, Heart),
            Card::new(Jack, Diamond),
            Card::new(Queen, Heart),
        ]);
        let king_high_card = Hand::new(vec![
            Card::new(King, Spade),
            Card::new(Seven, Club),
            Card::new(Eight, Club),
            Card::new(Jack, Diamond),
            Card::new(Queen, Heart),
        ]);

        assert!(ace_high_card != king_high_card);
        assert!(ace_high_card > king_high_card);
        assert!(king_high_card < ace_high_card);
    }

    #[test] fn a_pair_beats_a_high_card() {
        let pair_of_threes = Hand::new(vec![
            Card::new(Three, Spade),
            Card::new(Three, Heart),
            Card::new(Seven, Heart),
            Card::new(Jack, Diamond),
            Card::new(Queen, Heart),
        ]);
        let king_high_card = Hand::new(vec![
            Card::new(King, Spade),
            Card::new(Four, Club),
            Card::new(Eight, Club),
            Card::new(Jack, Diamond),
            Card::new(Queen, Heart),
        ]);

        assert!(pair_of_threes != king_high_card);
        assert!(pair_of_threes > king_high_card);
        assert!(king_high_card < pair_of_threes);
    }

    #[test] fn a_pair_beats_a_worse_pair() {
        let pair_of_threes = Hand::new(vec![
            Card::new(Three, Spade),
            Card::new(Three, Heart),
            Card::new(Seven, Heart),
            Card::new(Jack, Diamond),
            Card::new(Queen, Heart),
        ]);
        let pair_of_twos = Hand::new(vec![
            Card::new(Two, Spade),
            Card::new(Two, Heart),
            Card::new(Seven, Heart),
            Card::new(Jack, Diamond),
            Card::new(Queen, Heart),
        ]);

        assert!(pair_of_threes != pair_of_twos);
        assert!(pair_of_threes > pair_of_twos);
        assert!(pair_of_twos < pair_of_threes);
    }
}
