use std::fmt;
use std::slice::Iter;
use self::Rank::*;
use self::Suit::*;

#[derive(Clone, Copy)]
pub enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Clubs => write!(f, "Clubs"),
            Spades => write!(f, "Spades"),
            Diamonds => write!(f, "Diamonds"),
            Hearts => write!(f, "Hearts"),
        }
    }
}

impl Suit {
    pub fn iter() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Clubs, Spades, Hearts, Diamonds];
        SUITS.iter()
    }
}

#[derive(Clone, Copy)]
pub enum Rank {
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
    King,
    Ace
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Two => write!(f, "Two"),
            Three => write!(f, "Three"),
            Four => write!(f, "Four"),
            Five => write!(f, "Five"),
            Six => write!(f, "Six"),
            Seven => write!(f, "Seven"),
            Eight => write!(f, "Eight"),
            Nine => write!(f, "Nine"),
            Ten => write!(f, "Ten"),
            Jack => write!(f, "Jack"),
            Queen => write!(f, "Queen"),
            King => write!(f, "King"),
            Ace => write!(f, "Ace")
        }
    }
}

impl Rank {
    pub fn iter() -> Iter<'static, Rank> {
        static RANKS: [Rank; 13] = [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace];
        RANKS.iter()
    }
}

#[derive(Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}