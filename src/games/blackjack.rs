use crate::games::game::Game;
use crate::player::Player;
use crate::card::Rank;
use crate::card::{Card, Suit};

pub struct Blackjack<'a> {
	player: Player<'a>,
	dealer: Player<'a>,
}

impl Blackjack<'_> {
	pub fn new() -> Blackjack<'static> {
		Blackjack {
			player: Player::new("Player"),
			dealer: Player::new("Dealer")
		}
	}
}

impl Game for Blackjack<'_> {
	fn play(&mut self) {
		println!("Blackjack! {} vs {}", self.player.name, self.dealer.name);

		self.player.hand.push(Card {
			rank: Rank::Ten,
			suit: Suit::Hearts
		});

		self.player.hand.push(Card {
			rank: Rank::Ace,
			suit: Suit::Hearts
		});

		self.player.hand.push(Card {
			rank: Rank::Five,
			suit: Suit::Hearts
		});

		self.player.hand.push(Card {
			rank: Rank::Five,
			suit: Suit::Hearts
		});

		println!("{}", self.player.score());
	}
}

impl Player<'_> {
	fn score (&self) -> i32 {
		let mut score = 0;
		let mut aces = 0;

		for card in &self.hand {
			match card.rank {
				Rank::Two => score += 2,
				Rank::Three => score += 3,
				Rank::Four => score += 4,
				Rank::Five => score += 5,
				Rank::Six => score += 6,
				Rank::Seven => score += 7,
				Rank::Eight => score += 8,
				Rank::Nine => score += 9,
				Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => score += 10,
				Rank::Ace => aces += 1
			}
		}

		for _ in 0..aces {
			if score < 11 {
				score += 11;
			}
			else {
				score += 1;
			}
		}

		score
	}
}

