mod card;
mod cardpoints;
mod distribution;

mod games;

use card::{Card, Suit, Value};

fn main() {
    let teaser_card = Card::new(Suit::Spade, Value::Ace);
    println!("Welcome to Solitairey Confinement! {}", teaser_card);

    let game = games::diamond_mine::DiamondMine::new();
    println!("____\n{}", game);
    println!("{:?}", game.generate_actions());
}
