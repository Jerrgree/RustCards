pub mod card;
pub mod player;
pub mod games;
mod deck;
use deck::Deck;
use games::game::Game;
use games::blackjack::Blackjack;


fn main() {
    let game = &mut Blackjack::new();
    game.play();
}