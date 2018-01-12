#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

use card::{self, Card, Suit, Value};

#[derive(Debug)]
pub struct Poker {
    deck: Vec<Card>,
    array: [[Option<Card>; 5]; 5]
}

impl Poker {
    pub fn new() -> Self {
        let deck = card::deal();
        let array = [[None; 5]; 5];
        Self { deck, array }
    }
}

#[derive(Debug)]
pub enum Hand {
    Null,
    Pair,
    TwoPair,
    Flush,
    ThreeOfAKind,
    FullHouse,
    Straight,
    FourOfAKind,
    StraightFlush,
    // the so-called "royal flush" is an urban legend
}

impl Hand {
    fn score(&self) -> usize {
        match *self {
            Hand::Null => 0,
            Hand::Pair => 1,
            Hand::TwoPair => 3,
            Hand::Flush => 5,
            Hand::ThreeOfAKind => 6,
            Hand::FullHouse => 10,
            Hand::Straight => 12,
            Hand::FourOfAKind => 16,
            Hand::StraightFlush => 30,
        }
    }
}

pub fn evaluate(cards: &[Card]) -> Hand {
    let mut values = cards.iter().map(|card| card.value).collect::<Vec<_>>();
    values.sort();

    let suits = cards.iter().map(|card| card.suit).collect::<Vec<_>>();

    // check for two best multiples
    let mut counter = HashMap::new();
    for value in &values {
        let instances = counter.entry(value).or_insert(0);
        *instances += 1;
    }
    let mut multiples = counter.values().collect::<Vec<_>>();
    multiples.sort_by_key(|&multiple| -multiple);

    // check for flushness
    let flushness = suits.iter().all(|&suit| suit == suits[0]);

    // check for straightness
    let mut gaps = Vec::new();
    // XXX why are we so bad at stabilizing things (`slice pattern syntax is
    // experimental (see issue #23121)` if you try the obvious destructuring)
    for window in values.windows(2) {
        let card = window[0];
        let next = window[1];
        gaps.push(next.as_int() - card.as_int());
    }
    let straightness = gaps.iter().all(|&gap| gap == 1);

    match (straightness, flushness, *multiples[0], *multiples[1]) {
        (true, true, _, _) => Hand::StraightFlush,
        (_, _, 4, _) => Hand::FourOfAKind,
        (true, false, _, _) => Hand::Straight,
        (_, _, 3, 2) => Hand::FullHouse,
        (_, _, 3, _) => Hand::ThreeOfAKind,
        (false, true, _, _) => Hand::Flush,
        (_, _, 2, 2) => Hand::TwoPair,
        (_, _, 2, _) => Hand::Pair,
        _ => Hand::Null
    }
}
