mod card;
mod cardpoints;
mod distribution;

mod games;

use card::{Card, Suit, Value};

fn main() {
    let teaser_card = Card::new(Suit::Spade, Value::Ace);
    println!("{}", teaser_card);
}
