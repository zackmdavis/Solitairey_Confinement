#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use card::{self, Card, Suit, Color, Value};


pub struct Distribution {
    backing: HashMap<Card, usize>
}

impl Distribution {
    pub fn new() -> Self {
        let deck = card::deal();
        let mut distribution = Distribution {
            backing: HashMap::with_capacity(deck.len())
        };
        for card in deck {
            let frequency = distribution.backing.entry(card).or_insert(0);
            *frequency += 1;
        }
        distribution
    }

    pub fn set(&mut self, card: Card) {
        let mut rebacking = HashMap::new();
        rebacking.insert(card, 1);
        self.backing = rebacking;
    }

    pub fn decrement(&mut self, card: Card) {
        if let Entry::Occupied(mut o) = self.backing.entry(card) {
            *o.get_mut() -= 1;
        }
    }

    pub fn probability(&self, card: Card) -> f64 {
        let total: usize = self.backing.values().sum();
        *self.backing.get(&card).unwrap_or(&0) as f64 / (total as f64)
    }

    pub fn property_probability(&self, property: &Fn(Card) -> bool) -> f64 {
        let total: usize = self.backing.values().sum();
        let with_property: usize = self.backing.keys()
            .filter_map(|&card|
                        if property(card) {
                            Some(1)
                        } else {
                            None
                        })
            .sum();
        (with_property as f64)/(total as f64)
    }

    pub fn color_probability(&self, color: Color) -> f64 {
        self.property_probability(&(|card| card.suit.color() == color))
    }

    pub fn suit_probability(&self, suit: Suit) -> f64 {
        self.property_probability(&(|card| card.suit == suit))
    }

    pub fn value_probability(&self, value: Value) -> f64 {
        self.property_probability(&(|card| card.value == value))
    }

    pub fn entropy(&self) -> f64 {
        let values = self.backing.values().cloned().collect::<Vec<usize>>();
        let total = values.iter().sum::<usize>() as f64;
        values.iter().map(|d| {
            let f = (*d) as f64;
            -(f/total) * (f/total).log2()
        }).sum()
    }

}

#[cfg(test)]
#[macro_use]
macro_rules! assert_eq_within_epsilon {
    // crude edit of the canonical `assert_eq!`
    ($left:expr, $right:expr, $epsilon:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (*left_val - *right_val).abs() > $epsilon {
                    panic!("assertion failed: left and right not within ε \
                           (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concerning_intial_entropy() {
        let distribution = Distribution::new();
        assert_eq_within_epsilon!(5.7004397, distribution.entropy(), 0.0001);
    }

    #[test]
    fn concerning_inital_color_balance() {
        let distribution = Distribution::new();
        assert_eq_within_epsilon!(0.5, distribution.color_probability(Color::Red),
                                  0.0001);
    }
}
