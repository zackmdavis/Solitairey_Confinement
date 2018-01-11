#![allow(dead_code, unused_imports)]

use card::{self, Card, Suit, Value};

pub struct Address<'a> {
    row: usize,
    offset: usize,
    card: &'a Card
}

pub struct Escalator {
    deck: Vec<Card>,
    structure: Vec<Vec<Option<Card>>>
}

impl Escalator {
    pub fn new() -> Self {
        let mut deck = card::deal();
        let mut structure = Vec::new();
        for row_index in 0..7 {
            let mut row = Vec::new();
            for _ in 0..row_index+1 {
                row.push(deck.pop())
            }
            structure.push(row);
        }
        Self { deck, structure }
    }

    pub fn accessibles(&self) -> Vec<Address> {
        Vec::new(/* TODO */)
    }

}
