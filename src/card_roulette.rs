use rand::seq::SliceRandom;
use rand::thread_rng;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const PURPLE: &str = "\x1b[35m";
const RESET: &str = "\x1b[0m";

/// A struct representing the Card Roulette game.
///
/// This struct allows users to simulate multiple rounds of the Card Roulette game,
/// track scores for each player, and customize the number of players and rounds.
pub struct CardRoulette {
    /// The number of players in the game.
    pub num_players: usize,
    /// The number of rounds in the game.
    pub num_rounds: usize,
    /// The total number of cards in the game.
    pub num_cards: usize,
    /// The cards remaining in the deck.
    pub cards: Vec<usize>,
    /// The scores of each player.
    pub scores: Vec<usize>,
}

impl CardRoulette {
    /// Initializes a new `CardRoulette` instance.
    ///
    /// # Arguments
    ///
    /// * `num_players` - The number of players in the game.
    /// * `num_rounds` - The number of rounds in the game.
    ///
    /// # Returns
    ///
    /// A new `CardRoulette` instance.
    pub fn new(num_players: usize, num_rounds: usize) -> Self {
        let num_cards: usize = num_players * num_rounds;
        let mut cards: Vec<usize> = Vec::with_capacity(num_cards);
        for card in 1..=num_cards {
            cards.push(card);
        }
        Self {
            num_players,
            num_rounds,
            num_cards,
            cards,
            scores: vec![0; num_players],
        }
    }

    /// Resets the deck of cards for the game.
    pub fn reset_cards(&mut self) {
        self.cards.clear();
        for card in 1..=self.num_cards {
            self.cards.push(card);
        }
    }

    /// Performs a round of the game.
    ///
    /// # Arguments
    ///
    /// * `current_player` - The index of the first player to draw a card.
    /// * `deadly_card` - The card that is deadly.
    ///
    /// # Returns
    ///
    /// `true` if the round is over, `false` otherwise.
    pub fn play_round(&mut self, mut current_player: usize, deadly_card: usize) -> bool {
        let mut round_over: bool = false;
        for _ in 0..self.num_players {
            if self.cards.len() != 1 {
                let players_pick: usize = *self.cards.choose(&mut thread_rng()).unwrap();
                println!("Player {} draws {}", current_player + 1, players_pick);
                self.cards.retain(|&x| x != players_pick);
                if players_pick == deadly_card {
                    round_over = true;
                    println!("{}Player {} loses.{}", RED, current_player + 1, RESET);
                    self.scores[current_player] += 1;
                    break;
                }
                current_player = (current_player + 1) % self.num_players;
            } else {
                println!("{}Everyone survives!{}", GREEN, RESET);
                round_over = true;
                break;
            }
        }
        round_over
    }

    /// Performs an iteration of the game with a specified deadly card.
    ///
    /// # Arguments
    ///
    /// * `first_player` - The index of the first player to draw a card.
    /// * `deadly_card` - The card that is deadly.
    pub fn play_iteration(&mut self, first_player: usize, deadly_card: usize) {
        println!("{}Iteration {}{}", PURPLE, deadly_card, RESET);
        let mut round_over: bool = false;
        while !round_over {
            round_over = self.play_round(first_player, deadly_card);
        }
        if round_over {
            self.reset_cards();
        }
    }

    /// Plays the Card Roulette game.
    pub fn play_game(&mut self) {
        for deadly_card in 1..=self.num_cards {
            let first_player: usize = (deadly_card - 1) % self.num_players;
            self.play_iteration(first_player, deadly_card);
            println!();
        }
        println!("{}Results:{}", YELLOW, RESET);
        for (player, score) in self.scores.iter().enumerate() {
            println!("{}Player {} - {} points{}", YELLOW, player + 1, score, RESET);
        }
        println!();
    }
}

// /// The entry point of the application.
// pub fn main() {
//     let mut game: CardRoulette = CardRoulette::new(3, 3);
//     game.play_game();
//     println!("Results:");
//     for (player, &score) in game.scores.iter().enumerate() {
//         println!("Player {} - {} points", player + 1, score);
//     }
// }
