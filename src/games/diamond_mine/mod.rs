#![allow(dead_code, unused_imports)]

use card::{self, Card, Suit, Value, CardInPlay, Visibility};

#[derive(Debug)]
pub struct DiamondMine {
    pub foundation: Vec<Card>,
    pub tableau: [Vec<CardInPlay>; 13]
}

pub enum Action<'a> {
    ToFoundation(Card),
    OnTableau { from: usize, to: usize, cards: &'a [CardInPlay]}
}

impl DiamondMine {
    fn new() -> Self {
        let mut deck = card::deal();
        let mut mine = Self { foundation: Vec::new(),
                              // XXX: I'm pretty sure there's a rust-lang/rust
                              // issue to make this more ergonomic
                              tableau: [Vec::new(), Vec::new(), Vec::new(),
                                        Vec::new(), Vec::new(), Vec::new(),
                                        Vec::new(), Vec::new(), Vec::new(),
                                        Vec::new(), Vec::new(), Vec::new(),
                                        Vec::new()]};
        for pile in &mut mine.tableau {
            for _ in 0..3 {
                pile.push(deck.pop().expect("deck should not be empty")
                          .place_down())
            }
            pile.push(deck.pop().expect("deck should not be empty")
                          .place_up());
        }
        mine
    }



}
