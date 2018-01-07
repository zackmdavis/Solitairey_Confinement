#![allow(dead_code, unused_imports)]

use std::fmt;

use card::{self, Card, Suit, Value, CardInPlay, Visibility};

#[derive(Debug)]
pub struct DiamondMine {
    pub foundation: Vec<Card>,
    pub tableau: [Vec<CardInPlay>; 13]
}

impl fmt::Display for DiamondMine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.foundation.is_empty() {
            writeln!(f, "□")?
        } else {
            writeln!(f, "{}", self.foundation[self.foundation.len()-1])?
        }
        writeln!(f)?;
        for pile in &self.tableau {
            for &card in pile {
                match card.look() {
                    Some(card) => write!(f, "{}", card)?,
                    None => write!(f, "█")?
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}


pub enum Action<'a> {
    ToFoundation(Card),
    OnTableau { from: usize, to: usize, cards: &'a [CardInPlay]}
}

impl DiamondMine {
    pub fn new() -> Self {
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
