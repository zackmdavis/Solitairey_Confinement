#![allow(dead_code, unused_imports)]

use std::iter;

use card::{self, Card, Suit, Value};

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub struct Motion {
    card: Card,
    from: Location,
    to: Location,
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

    pub fn find(&self, target: Card) -> Location {
        if target.value == Value::Ace {
            panic!("there are no Aces in this game");
        }
        for (i, row) in self.structure.iter().enumerate() {
            for (j, slot) in row.iter().enumerate() {
                if let Some(sitting_card) = *slot {
                    if sitting_card == target {
                        return Location(i, j);
                    }
                }
            }
        }
        panic!("we couldn't find the card?!");
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

    #[cfg(XXX_broken_does_not_compile)] // TODO: fix it
    pub fn available_motions(&self) -> Vec<Motion> {
        let gaps = self.gap_locations();
        gaps.iter().flat_map(|&gap| {
            if let Some(prior_slot) = gap.previous().map(|prior| self.look(prior)) {
                if let Some(prior_card) = prior_slot {
                    if let Some(successor_card) = prior_card.successor() {
                        let motion = Motion {
                            card: successor_card,
                            from: self.find(successor_card),
                            to: gap
                        };
                        vec![motion].iter()
                    } else {
                        *&[].iter()
                    }
                } else {
                    *&[].iter()
                }
            } else {
                assert!(gap.1 == 0);
                // XXX TODO: don't let a deuce be moved if it's already in place
                let motions = Suit::suits().iter()
                    .map(|&suit| Card { suit, value: Value::Two })
                    .map(|two| Motion {
                        card: two,
                        from: self.find(two),
                        to: gap
                    }).collect::<Vec<_>>();
                motions.iter()
            }
        }).cloned().collect()
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
