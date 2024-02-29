use crate::card::{Rank, Suit, Card};
use std::vec::Vec;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::<Card>::with_capacity(52);

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                let card = Card {
                    rank: *rank,
                    suit: *suit
                };
                cards.push(card);
            }
        }

        Deck {
            cards: cards
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }
}
