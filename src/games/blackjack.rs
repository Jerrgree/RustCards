use crate::games::game::Game;
use crate::player::Player;

pub struct Blackjack {
	player: Player,
	dealer: Player,
}

impl Blackjack {
	pub fn new() -> Blackjack {
		Blackjack {
			player: Player::new("Player".to_string()),
			dealer: Player::new("Dealer".to_string())
		}
	}
}

impl Game for Blackjack {
	fn play(&self) {
		println!("Blackjack!")
	}
}