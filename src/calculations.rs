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

#[cfg(not(test))]
pub fn chance_of_winning(my_pocket: Hand, community_cards: Hand, other_players: usize) -> (f32, f32) {
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
    (wins / total, confidence_interval(wins, total))
}

static CONFIDENCE: f32 = 0.95;
use std::num::Float;

fn confidence_interval(successes: f32, observed: f32) -> f32 {
    let z = percent_std_normal(1.0 - (1.0 - CONFIDENCE) / 2.0);
    let p = successes / observed;
    z * (1.0 / observed * p * (1.0 - p)).sqrt()
}

use self::core::f32::consts::{SQRT2, FRAC_2_SQRTPI};

fn percent_std_normal(x: f32) -> f32 {
    let mut theory = 0.0;
    let mut change = 1.0 * 16.0;

    loop {
        if cdf_std_normal(theory) < x {
            theory += change;
        } else {
            theory -= change;
        }
        change /= 2.0;
        if change < 1.0 / 16.0.exp2() {
            break;
        }
    }
    theory
}

fn cdf_std_normal(x: f32) -> f32 {
    (1.0 + erf(x / SQRT2)) / 2.0
}

fn erf(x: f32) -> f32 {
    FRAC_2_SQRTPI * range(0, 7)
        .map(|n| {
            let product = range(1, n + 1).map(|k| - x.powi(2) / k.to_f32().unwrap()).fold(1.0, |prod, value| prod * value);
            x / (2 * n + 1).to_f32().unwrap() * product.to_f32().unwrap()
        })
        .fold(0.0, |sum, value| sum + value)
}

#[cfg(test)]
mod tests {
    use super::{confidence_interval};
    use std::num::Float;

    #[test] fn confidence_interval_works_for_exact_successes_and_observed() {
        assert_eq!(confidence_interval(1.0, 1.0), 0.0);
    }

    #[test] fn confidence_interval_works_for_arbitrary_successes_and_observed() {
        assert!((confidence_interval(670.0, 1000.0) - 0.0291).abs() < 0.0005);
    }
}
