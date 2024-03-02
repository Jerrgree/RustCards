pub mod card;
pub mod player;
pub mod games;
mod deck;
use deck::Deck;
use games::game::Game;
use games::blackjack::Blackjack;


fn main() {
    let game: &dyn Game = &Blackjack::new();
    game.play();

    let mut deck = Deck::new();
    deck.shuffle();

    while let Some(card) = deck.draw() {
        println!("{card}");
    }
}