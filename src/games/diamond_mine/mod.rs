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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Action {
    ToFoundation(Card),
    OnTableau { from: usize, to: usize, cards: Vec<Card>}
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

    pub fn visible_in_pile(&self, pile_index: usize) -> Vec<Card> {
        self.tableau[pile_index].iter()
            .filter_map(|card_in_play| card_in_play.look())
            .collect()
    }

    pub fn generate_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        for from_index in 0..13 {
            for to_index in 0..13 {
                let from_visible = self.visible_in_pile(from_index);
                let to_visible = self.visible_in_pile(to_index);
                if to_visible.is_empty() {
                    for (i, card) in from_visible.iter().enumerate() {
                        actions.push(Action::OnTableau {
                            from: from_index,
                            to: to_index,
                            cards: from_visible[i..].to_vec()
                        })
                    }
                } else {
                    let destination = self.tableau[to_index][self.tableau[to_index].len()-1]
                        .look().expect("card should be face-up");
                    if destination.suit == Suit::Diamond {
                        continue;
                    }
                    for (i, card) in from_visible.iter().enumerate() {
                        if card.suit == Suit::Diamond {
                            break;
                        }
                        if destination.value.as_int() - card.value.as_int() == 1 {
                            actions.push(Action::OnTableau {
                                from: from_index,
                                to: to_index,
                                cards: from_visible[i..].to_vec()
                            })
                        }
                    }
                }
            }
        }
        actions
    }

    pub fn apply_action(&mut self, action: Action) { /* TODO */ }

}
