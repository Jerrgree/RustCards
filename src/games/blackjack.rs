use crate::games::game::Game;
use crate::player::Player;
use crate::card::Rank;

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
	fn play(&self) {
		println!("Blackjack! {} vs {}", self.player.name, self.dealer.name)
	}
}

impl Player<'_> {
	fn score (&self) -> i32 {
		let mut score = 0;
		// Score the non-aces first, and then talley up the aces
		// Would technically be faster to do one loop, count the number of aces, and then score off of those later
		// But I wanted to play with closures and filters some
		// Also need to figure out if clone is really the best way to interate through a vector
		let non_aces = &self.hand.clone().into_iter().filter(|c| match c.rank { Rank::Ace => false, _ => true }).collect::<Vec<_>>();
		for card in non_aces {
			score += 1;
		}

		let aces = &self.hand.clone().into_iter().filter(|c| match c.rank { Rank::Ace => true, _ => false }).collect::<Vec<_>>();
		for card in aces {
			score += 1;
		}

		score
	}
}

