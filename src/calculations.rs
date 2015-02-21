use hand::Hand;

static WHOLE_DECK: [&'static str; 52] = [
    "AD", "2D", "3D", "4D", "5D", "6D", "7D", "8D", "9D", "0D", "JD", "QD", "KD",
    "AC", "2C", "3C", "4C", "5C", "6C", "7C", "8C", "9C", "0C", "JC", "QC", "KC",
    "AH", "2H", "3H", "4H", "5H", "6H", "7H", "8H", "9H", "0H", "JH", "QH", "KH",
    "AS", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "0S", "JS", "QS", "KS",
];

static SAMPLES: usize = 1000;

use std::rand::{thread_rng, Rng};

extern crate core;
use self::core::num::ToPrimitive;

pub fn chance_of_winning(my_pocket: Hand, community_cards: Hand, other_players: usize) -> (f32, bool) {
    let mut rng = thread_rng();
    let deck = WHOLE_DECK.iter()
                    .fold(String::new(), |string, card| string + " " + card)
                    .trim()
                    .parse::<Hand>()
                    .ok()
                    .unwrap();
    let remaining_deck = deck - my_pocket.clone() - community_cards.clone();
    let results = range(0, SAMPLES)
                  .map(|_| {
                      let mut deck_cards = remaining_deck.cards();
                      let mut cards = deck_cards.as_mut_slice();
                      rng.shuffle(cards);
                      let community_cards_needed = 5 - community_cards.cards().len();
                      let community_cards = community_cards.clone() + Hand::new(cards.iter().take(community_cards_needed).map(|&card| card).collect());
                      range(0, other_players)
                      .map(|player_index| {
                          Hand::new(cards
                                    .iter()
                                    .skip(community_cards_needed + 2 * player_index)
                                    .take(2)
                                    .map(|&card| card)
                                    .collect())
                      })
                      .all(|opponents_pocket| {
                          (my_pocket.clone() + community_cards.clone()) > (opponents_pocket + community_cards.clone())
                      })
                  }).collect::<Vec<bool>>();
    let total: f32 = results.iter().fold(0.0, |sum, _| sum + 1.0);
    let wins: f32 = results.iter().filter(|result| **result).fold(0.0, |sum, _| sum + 1.0);
    let chance = wins / total;
    (chance, SAMPLES.to_f32().unwrap() * chance > 5.0)
}
