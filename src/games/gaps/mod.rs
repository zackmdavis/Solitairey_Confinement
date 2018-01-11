#![allow(dead_code, unused_imports)]

use card::{self, Card, Suit, Value};

#[derive(Debug)]
pub struct Location(usize, usize);

impl Location {
    pub fn previous(&self) -> Option<Self> {
        if self.1 == 0 {
            None
        } else {
            Some(Location(self.0, self.1 - 1))
        }
    }

    pub fn following(&self) -> Option<Self> {
        if self.1 == 13 {
            None
        } else {
            Some(Location(self.0, self.1 + 1))
        }
    }
}

#[derive(Debug)]
pub struct Gaps {
    structure: Vec<Vec<Option<Card>>>,
    remaining_redeals: u8,
}

#[derive(Debug)]
pub enum Action {
    Motion { card: Card, from: Location, to: Location },
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

    pub fn look(&self, location: Location) -> Option<&Card> {
        self.structure[location.0][location.1].as_ref()
    }

    pub fn gap_locations(&self) -> Vec<Location> {
        let mut locations = Vec::new();
        for (i, row) in self.structure.iter().enumerate() {
            for (j, &slot) in row.iter().enumerate() {
                if let None = slot {
                    locations.push(Location(i, j));
                }
            }
        }
        locations
    }

    pub fn available_actions(&self) -> Vec<Action> {
        let gaps = self.gap_locations();
        let _motions = gaps.iter().map(|gap| {
            let _card_maybe = gap.previous().map(|prior| self.look(prior));
            // XXX TODO: `Card.succesor` needs to be two different methods, to
            // disambiguate between wrapping (appropriate to e.g., Diamond Mine
            // foundation) and nonwrapping (appropriate here) interpretation
        });
        Vec::new(/* TODO */)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concerning_gap_detection() {
        let game = Gaps::new();
        assert_eq!(4, game.gap_locations().len());
    }

}
