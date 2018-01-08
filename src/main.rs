mod card;
mod cardpoints;
mod distribution;

mod games;

use card::{Card, Suit, Value};
use games::diamond_mine;

fn main() {
    let teaser_card = Card::new(Suit::Spade, Value::Ace);
    println!("Welcome to Solitairey Confinement! {}", teaser_card);

    let mut game = diamond_mine::DiamondMine::new();
    let mut player = diamond_mine::player::Player::new();

    loop {
        println!("{}", game);
        let mut actions: Vec<diamond_mine::Action> = game.generate_actions();
        println!("{} actions available", actions.len());

        match player.select_action(actions) {
            Some(action) => game.apply_action(action),
            None => {
                println!("player resigns");
                break;
            }
        }
    }

    println!("Final game state:\n\n{}", game);
    println!("Foundation size: {}", game.foundation.len());
}
