use card::{Card, Value};
use card::Value::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug, Eq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        Hand { cards: cards }
    }

    fn categorize(&self) -> HandRank {
        match (self.is_straight(), self.is_flush()) {
            (true, true) => HandRank::StraightFlush,
            (true, false) => HandRank::Straight,
            (false, true) => HandRank::Flush,
            (false, false) => self.non_straight_flush_categorize(),
        }
    }

    fn non_straight_flush_categorize(&self) -> HandRank {
        let values_with_more_than_one = self.cards.iter()
            .map(|card| card.value())
            .filter(|&value| {
                self.cards.iter().filter(|inner_card| value == inner_card.value()).count() > 1
            }).fold(HashSet::new(), |mut set, value| {
                set.insert(value);
                set
            });
        match values_with_more_than_one.len() {
            2 => {
                match self.cards.iter().filter(|card| values_with_more_than_one.contains(card.value())).collect::<Vec<_>>().len() {
                     5 => HandRank::FullHouse,
                     _ => HandRank::TwoPair ,
                 }
            },
            1 => {
                match self.cards.iter().filter(|card| values_with_more_than_one.contains(card.value())).collect::<Vec<_>>().len() {
                    4 => HandRank::FourOfAKind,
                    3 => HandRank::ThreeOfAKind,
                    _ => HandRank::Pair,
                }
            },
            _ => HandRank::HighCard,
        }
    }

    fn is_straight(&self) -> bool {
        let mut values = self.cards.iter().map(|card| card.value()).collect::<Vec<&Value>>();
        values.sort();

        let mut value_iter = values.iter().map(|&value| value);
        match (value_iter.next(), value_iter.next(), value_iter.next(), value_iter.next(), value_iter.next()) {
            (Some(&Two), Some(&Three), Some(&Four), Some(&Five), Some(&Ace)) => true,
            (Some(&Two), Some(&Three), Some(&Four), Some(&Five), Some(&Six)) => true,
            (Some(&Three), Some(&Four), Some(&Five), Some(&Six), Some(&Seven)) => true,
            (Some(&Four), Some(&Five), Some(&Six), Some(&Seven), Some(&Eight)) => true,
            (Some(&Five), Some(&Six), Some(&Seven), Some(&Eight), Some(&Nine)) => true,
            (Some(&Six), Some(&Seven), Some(&Eight), Some(&Nine), Some(&Ten)) => true,
            (Some(&Seven), Some(&Eight), Some(&Nine), Some(&Ten), Some(&Jack)) => true,
            (Some(&Eight), Some(&Nine), Some(&Ten), Some(&Jack), Some(&Queen)) => true,
            (Some(&Nine), Some(&Ten), Some(&Jack), Some(&Queen), Some(&King)) => true,
            (Some(&Ten), Some(&Jack), Some(&Queen), Some(&King), Some(&Ace)) => true,
            _ => false,
        }
    }

    fn is_flush(&self) -> bool {
        let first_suit = self.cards[0].suit();
        self.cards.iter().all(|card| card.suit() == first_suit)
    }

    fn most_common_values(&self) -> Vec<&Value> {
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
        let mut tuples = vec![];
        for (value, count) in mapped.into_iter() {
            tuples.push((value, count));
        }
        tuples.sort_by(|&(value_left, count_left), &(value_right, count_right)| {
            match count_right.cmp(&count_left) {
                Ordering::Equal => value_right.cmp(&value_left),
                other => other,
            }
        });
        let mut values = tuples.iter().map(|&(value, _)| value).collect::<Vec<&Value>>();
        if self.is_straight() && values.contains(&&Ace) && values.contains(&&Five) {
            let removed = values.remove(0);
            values.push(removed);
        }
        values
    }

    fn sub_hands(&self) -> Vec<Hand> {
        self.cards.iter().map(|card_to_ignore| {
            Hand::new(self.cards.iter()
                      .filter(|&card| card != card_to_ignore)
                      .map(|&card| card)
                      .collect())
        }).collect()
    }
}

impl FromStr for Hand {
    type Err = HandParseErr;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let cards: Vec<Option<Card>> = s.split_str(" ")
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
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cards.len() > 5 {
            true => {
                let my_sub_hands = self.sub_hands();
                let other_sub_hands = other.sub_hands();

                let my_best_hand = my_sub_hands.iter().max();
                let other_best_hand = other_sub_hands.iter().max();

                my_best_hand.partial_cmp(&other_best_hand)
            },
            false => {
                match self.categorize().partial_cmp(&other.categorize()) {
                    Some(Ordering::Equal) => {
                        self.most_common_values().partial_cmp(&other.most_common_values())
                    },
                    other => other
                }
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(&other) {
            Some(order) => order,
            None => Ordering::Equal
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hand;

    fn parse_hand(s: &str) -> Hand {
        s.parse().ok().unwrap()
    }

    fn assert_hand_beats(winner: Hand, loser: Hand) {
        assert!(winner != loser);
        assert!(winner > loser);
        assert!(loser < winner);
    }

    #[test] fn high_card_equals_high_card() {
        let high_card = parse_hand("5D 6H 7H AS JD");
        let other_high_card = parse_hand("5H 6C 7C AS JD");
        assert!(high_card == other_high_card);
    }

    #[test] fn high_card_beats_a_lower_high_card() {
        assert_hand_beats(parse_hand("AS 6H 7H JD QH"), parse_hand("KS 7C 8C JD QH"));
    }

    #[test] fn tied_high_card_decides_with_lower_cards() {
        assert_hand_beats(parse_hand("5D 6H 7H AS JD"), parse_hand("4H 6C 7C AS JD"));
    }

    #[test] fn a_pair_beats_a_high_card() {
        assert_hand_beats(parse_hand("3S 3H 7H JD QH"), parse_hand("KS 4C 8C JD QH"));
    }

    #[test] fn a_pair_beats_a_worse_pair() {
        assert_hand_beats(parse_hand("3S 3H 7H JD QH"), parse_hand("2S 2H 7H JD QH"));
    }

    #[test] fn tied_pairs_decide_with_high_card() {
        assert_hand_beats(parse_hand("3S 3H 7H JD QH"), parse_hand("3S 3H 6H JD QH"));
    }

    #[test] fn two_pair_beats_a_pair() {
        assert_hand_beats(parse_hand("2S 2H 3H 3D QH"), parse_hand("6S 6H 7H JD QH"));
    }

    #[test] fn two_pair_beats_a_lower_two_pair() {
        assert_hand_beats(parse_hand("4S 4H 5H 5D QH"), parse_hand("2S 2H 3H 3D QH"));
    }

    #[test] fn two_pair_tied_first_decides_with_lower() {
        assert_hand_beats(parse_hand("4S 4H 5H 5D QH"), parse_hand("2S 2H 5H 5D QH"));
    }

    #[test] fn two_pair_tied_all_decides_with_last_card() {
        assert_hand_beats(parse_hand("4S 4H 5H 5D AH"), parse_hand("4S 4H 5H 5D KH"));
    }

    #[test] fn three_of_a_kind_beats_two_pair() {
        assert_hand_beats(parse_hand("4S 4H 4H 5D AH"), parse_hand("4S 4H 5H 5D KH"));
    }

    #[test] fn three_of_a_kind_beats_a_lower_three_of_a_kind() {
        assert_hand_beats(parse_hand("4S 4H 4H 5D AH"), parse_hand("3S 3H 3H 5D KH"));
    }

    #[test] fn three_of_a_kind_beats_a_tied_three_of_a_kind_with_remaining_cards() {
        assert_hand_beats(parse_hand("4S 4H 4H 5D AH"), parse_hand("4S 4H 4H 3D AH"));
    }

    #[test] fn straight_beats_a_three_of_a_kind() {
        assert_hand_beats(parse_hand("2S 3H 4H 5D 6H"), parse_hand("4S 4H 4H 3D AH"));
    }

    #[test] fn ace_high_straight_beats_five_high() {
        assert_hand_beats(parse_hand("0S JS QH KD AS"), parse_hand("AH 2S 3H 4H 5D"));
    }

    #[test] fn six_high_straight_beats_five_high() {
        assert_hand_beats(parse_hand("2H 3S 4H 5H 6D"), parse_hand("AH 2S 3H 4H 5D"));
    }

    #[test] fn straight_with_ace_wrapped_around_is_a_high_card() {
        assert_hand_beats(parse_hand("4S 4H 4H 3D AH"), parse_hand("QS KH AH 2S 3H"));
    }

    #[test] fn flush_beats_a_straight() {
        assert_hand_beats(parse_hand("AH QH 2H 6H 7H"), parse_hand("2S 3H 4H 5D 6H"));
    }

    #[test] fn flush_is_tied_despite_suits() {
        assert!(parse_hand("AH QH 2H 6H 7H") == parse_hand("AD QD 2D 6D 7D"));
    }

    #[test] fn flushes_are_decided_by_values() {
        assert_hand_beats(parse_hand("AH QH 3H 6H 7H"), parse_hand("AD QD 2D 6D 7D"));
    }

    #[test] fn full_house_beats_a_flush() {
        assert_hand_beats(parse_hand("0H 0S 0D AS AD"), parse_hand("AH QH 2H 6H 7H"));
    }

    #[test] fn full_house_uses_three_cards_first() {
        assert_hand_beats(parse_hand("0H 0S 0D KS KD"), parse_hand("9H 9S 9D AS AD"));
    }

    #[test] fn full_house_uses_two_cards_last() {
        assert_hand_beats(parse_hand("0H 0S 0D JS JD"), parse_hand("0H 0S 0D 8S 8D"));
    }

    #[test] fn four_of_a_kind_beats_a_full_house() {
        assert_hand_beats(parse_hand("0H 0S 0D 0C AD"), parse_hand("JH JS JD 8S 8D"));
    }

    #[test] fn four_of_a_kind_uses_final_card_for_ties() {
        assert_hand_beats(parse_hand("0H 0S 0D 0C 6D"), parse_hand("0H 0S 0D 0C 5D"));
    }

    #[test] fn straight_flush_beats_a_four_of_a_kind() {
        assert_hand_beats(parse_hand("AH 2H 3H 4H 5H"), parse_hand("JH JS JD JC 8D"));
    }

    #[test] fn straight_flush_beats_a_lower_straight_flush() {
        assert_hand_beats(parse_hand("2H 3H 4H 5H 6H"), parse_hand("AH 2H 3H 4H 5H"));
    }

    #[test] fn seven_card_hand_high_cards() {
        assert_hand_beats(parse_hand("2H 3H 7C 5H 6H 9D JS"), parse_hand("2H 3H 7C 5H 6H 9D 0S"));
    }

    #[test] fn seven_card_hand_flush() {
        assert_hand_beats(parse_hand("2H 3H 0H 5H 6H 9D KS"), parse_hand("2H 3H 7C 5H 6H 9D AS"));
    }
}
