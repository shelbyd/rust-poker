use card::{Card, Value};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

enum HandRank {
    HighCard,
    Pair,
    TwoPair,
}

impl HandRank {
    fn _value(&self) -> u8 {
        match self {
            &HandRank::HighCard => 0,
            &HandRank::Pair => 1,
            &HandRank::TwoPair => 2,
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
        let values_with_more_than_one = self.cards.iter()
            .map(|card| card.value())
            .filter(|&value| {
                self.cards.iter().filter(|inner_card| value == inner_card.value()).count() > 1
            }).fold(HashSet::new(), |mut set, value| {
                set.insert(value);
                set
            });
        match values_with_more_than_one.len() {
            2 => HandRank::TwoPair,
            1 => HandRank::Pair,
            _ => HandRank::HighCard,
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
            .filter(|&(_, count)| count == max_count)
            .max_by(|&(&value, _)| value) {
                Some((&v, _)) => Some(v),
                None => None
            }
    }
}

impl FromStr for Hand {
    type Err = HandParseErr;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let cards: Vec<Option<Card>> = s.split_str(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|card_string| {
                match card_string.parse() {
                    Ok(result) => Some(result),
                    _ => None,
                }
            })
            .collect();
        let any_failed = cards.iter().any(|card| card.is_none());
        match any_failed {
            true => Err(HandParseErr::Err),
            false => Ok(Hand::new(cards.into_iter().map(|card| card.unwrap()).collect())),
        }
    }
}

enum HandParseErr {
    Err
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
    use super::Hand;

    fn parse_hand(s: &str) -> Hand {
        s.parse().ok().unwrap()
    }

    #[test] fn high_card_equals_high_card() {
        let high_card = parse_hand("5D 6H 7H AS JD");
        let other_high_card = parse_hand("5H 6C 7C AS JD");
        assert!(high_card == other_high_card);
    }

    #[test] fn high_card_beats_a_lower_high_card() {
        let ace_high_card = parse_hand("AS 6H 7H JD QH");
        let king_high_card = parse_hand("KS 7C 8C JD QH");

        assert!(ace_high_card != king_high_card);
        assert!(ace_high_card > king_high_card);
        assert!(king_high_card < ace_high_card);
    }

    #[test] fn a_pair_beats_a_high_card() {
        let pair_of_threes = parse_hand("3S 3H 7H JD QH");
        let king_high_card = parse_hand("KS 4C 8C JD QH");

        assert!(pair_of_threes != king_high_card);
        assert!(pair_of_threes > king_high_card);
        assert!(king_high_card < pair_of_threes);
    }

    #[test] fn a_pair_beats_a_worse_pair() {
        let pair_of_threes = parse_hand("3S 3H 7H JD QH");
        let pair_of_twos = parse_hand("2S 2H 7H JD QH");

        assert!(pair_of_threes != pair_of_twos);
        assert!(pair_of_threes > pair_of_twos);
        assert!(pair_of_twos < pair_of_threes);
    }

    #[test] fn two_pair_beats_a_pair() {
        let two_pair = parse_hand("2S 2H 3H 3D QH");
        let pair_of_sixes = parse_hand("6S 6H 7H JD QH");

        assert!(two_pair != pair_of_sixes);
        assert!(two_pair > pair_of_sixes);
        assert!(pair_of_sixes < two_pair);
    }

    #[test] fn two_pair_beats_a_lower_two_pair() {
        let high_two_pair = parse_hand("4S 4H 5H 5D QH");
        let low_two_pair = parse_hand("2S 2H 3H 3D QH");

        assert!(high_two_pair != low_two_pair);
        assert!(high_two_pair > low_two_pair);
        assert!(low_two_pair < high_two_pair);
    }
}
