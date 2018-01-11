#![allow(dead_code, unused_imports)]

use card::{self, Card, Suit, Value};


#[derive(Debug)]
pub struct Gaps {
    structure: Vec<Vec<Option<Card>>>,
    remaining_redeals: u8,
}

#[derive(Debug)]
pub enum Action {
    Motion { card: Card, from: (u8, u8), to: (u8, u8) },
    Redeal
}

impl Gaps {
    pub fn new() -> Self {
        let mut deck = card::deal();
        let mut structure = Vec::new();
        for _ in 0..4 {
            let mut row = Vec::new();
            for _ in 0..13 {
                let card = deck.pop().expect("deck should not be empty yet");
                if card.value != Value::Ace {
                    row.push(Some(card))
                } else {
                    row.push(None)
                }
            }
            structure.push(row);
        }
        Gaps { structure, remaining_redeals: 2 }
    }

    #[cfg(TODO_borrow_checker_troubles)]
    pub fn gap_locations(&self) -> Vec<(usize, usize)> {
        self.structure.iter().enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(|(j, &slot)| match slot {
                    Some(_) => None,
                    None => Some((i, j))
                })
            }).collect()
    }

    #[cfg(TODO_borrow_checker_troubles)]
    pub fn available_actions(&self) -> Vec<Action> {
        let gaps = self.gap_locations();
        Vec::new(/* TODO */)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(TODO_borrow_checker_troubles)]
    #[test]
    fn concerning_gap_detection() {
        let game = Gaps::new();
        assert_eq!(4, game.gap_locations().len());
    }

}
