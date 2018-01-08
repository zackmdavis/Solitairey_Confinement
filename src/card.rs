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

    pub fn as_int(&self) -> isize {
        match *self {
            Value::Ace => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13
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

    pub fn place(self, visibility: Visibility) -> CardInPlay {
        CardInPlay::new(self, visibility)
    }

    pub fn place_down(self) -> CardInPlay {
        self.place(Visibility::FaceDown)
    }

    pub fn place_up(self) -> CardInPlay {
        self.place(Visibility::FaceUp)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Visibility {
    FaceUp,
    FaceDown
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CardInPlay {
    card: Card,
    pub visibility: Visibility
}

impl CardInPlay {
    pub fn new(card: Card, visibility: Visibility) -> Self {
        Self { card, visibility }
    }

    pub fn flip_up(&mut self) -> bool {
        match self.visibility {
            Visibility::FaceUp => false,
            Visibility::FaceDown => {
                self.visibility = Visibility::FaceUp;
                true
            }
        }
    }

    pub fn flip_down(&mut self) -> bool {
        match self.visibility {
            Visibility::FaceUp => {
                self.visibility = Visibility::FaceDown;
                true
            },
            Visibility::FaceDown => false,
        }
    }

    pub fn look(&self) -> Option<Card> {
        match self.visibility {
            Visibility::FaceUp => Some(self.card),
            Visibility::FaceDown => None
        }
    }
}


pub fn deal() -> Vec<Card> {
    let mut deck = Vec::new();
    for suit in Suit::suits() {
        for value in Value::values() {
            deck.push(Card::new(suit, value));
        }
    }
    // XXX need to shuffle! I don't have internet access right now, though
    // (preventing me from grabbing `rand` from crates.io)
    //
    // And (following the scenario described in the README), I kind of like the
    // conceit of this being a no-dependencies project (if you had internet
    // access and wanted to procrastinate, you'd probably go for a more
    // enticing distraction than the solitaire that happened to come with your
    // OS)
    //
    // maybe implement Fisher–Yates??
    deck
}
