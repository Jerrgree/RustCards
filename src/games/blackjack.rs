use crate::games::game::Game;
use crate::player::Player;
use crate::card::Rank;
use crate::deck::Deck;

pub struct Blackjack<'a> {
	player: Player<'a>,
	dealer: Player<'a>,
	deck: Deck,
}

impl Blackjack<'_> {
	pub fn new() -> Blackjack<'static> {
		Blackjack {
			player: Player::new("Player"),
			dealer: Player::new("Dealer"),
			deck: Deck::new(),
		}
	}

	fn print_player_hand(&self) {
		let message = &self.player.hand.iter()
			.fold(String::from(""), |s, c| s + &c.to_string() + ", ");
		println!("{}", message);
	}

	fn print_dealer_hand(&self) {
		let firstCard = &self.dealer.hand[0];
		println!("Dealer is showing: {firstCard}");
	}
}

impl Game for Blackjack<'_> {
	fn play(&mut self) {
		println!("Blackjack! {} vs {}", self.player.name, self.dealer.name);

		self.deck.shuffle();

		self.player.hand.push(self.deck.draw().expect("Initial deal should not be empty"));
		self.dealer.hand.push(self.deck.draw().expect("Initial deal should not be empty"));
		self.player.hand.push(self.deck.draw().expect("Initial deal should not be empty"));
		self.dealer.hand.push(self.deck.draw().expect("Initial deal should not be empty"));

		self.print_player_hand();
		self.print_dealer_hand();
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