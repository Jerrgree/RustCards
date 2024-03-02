use crate::deck::Deck;
use crate::games::game::Game;
use crate::player::Player;
use crate::card::Rank;
use text_io::read;

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

	fn print_dealer_hand(&self) {
		let first_card = &self.dealer.hand[0];
		println!("Dealer is showing: {first_card}");
	}
}

impl Game for Blackjack<'_> {
	fn play(&mut self) {
		let mut continue_play = true;
		println!("Blackjack! {} vs {}", self.player.name, self.dealer.name);

		self.deck.shuffle();

		self.player.hand.push(self.deck.draw().expect("Initial deal should not be empty"));
		self.dealer.hand.push(self.deck.draw().expect("Initial deal should not be empty"));
		self.player.hand.push(self.deck.draw().expect("Initial deal should not be empty"));
		self.dealer.hand.push(self.deck.draw().expect("Initial deal should not be empty"));

		while self.player.score() < 22 && continue_play == true {
			print_player_hand(&self.player);
			self.print_dealer_hand();

			print_prompts();

			let word: String = read!();

			match word.as_str() {
				"h" => self.player.hand.push(self.deck.draw().expect("Should not run out of cards")),
				"s" => continue_play = false,
				_ => println!("Unrecognized option")
			}
		}

		if self.player.score() > 21 {
			println!("Oh no, you busted!");
		} else if self.player.score() == 21 && self.player.hand.len() == 2 {
			println!("Blackjack!");
		}

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

fn print_prompts() {
	println!("You may [h]it, or [s]tay")
}

fn print_player_hand(player: &Player) {
	let message = player.hand.iter()
		.fold(format!("{} has: ", player.name), |s, c| s + &c.to_string() + ", ");
	println!("{}", message);
}
