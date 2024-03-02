use crate::games::game::Game;
use crate::player::Player;

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
		println!("Blackjack!")
	}
}