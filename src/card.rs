#![allow(dead_code)]

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Black
}

impl Suit {
    pub fn suits() -> Vec<Self> {
        vec![Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club]
    }

    pub fn color(&self) -> Color {
        match *self {
            Suit::Heart | Suit::Diamond => Color::Red,
            Suit::Spade | Suit::Club => Color::Black
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Value {
    pub fn values() -> Vec<Self> {
        vec![Value::Ace, Value::Two, Value::Three, Value::Four, Value::Five,
             Value::Six, Value::Seven, Value::Eight, Value::Nine, Value::Ten,
             Value::Jack, Value::Queen, Value::King]
    }

    pub fn royal(&self) -> bool {
        match *self {
            Value::Jack | Value::Queen | Value::King => true,
            _ => false
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self { suit, value }
    }
}

pub fn deal() -> Vec<Card> {
    let mut deck = Vec::new();
    for suit in Suit::suits() {
        for value in Value::values() {
            deck.push(Card::new(suit, value));
        }
    }
    deck
}
