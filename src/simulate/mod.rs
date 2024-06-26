mod manage_results;

use uuid::Uuid;
use manage_results::*;
use crate::card_roulette::CardRoulette;

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

/// Simulate games of Card Roulette and save the results.
///
/// # Arguments
///
/// * `num_games` - Number of games to simulate.
/// * `num_players` - Number of players in each game.
/// * `num_rounds` - Number of rounds in each game.
pub fn simulate_games(num_games: usize, num_players: usize, num_rounds: usize) {
    create_results_directory();
    let players_dir: String = create_player_directory(num_players);
    let simulation_id: String = Uuid::new_v4().to_string();

    for game_number in 1..=num_games {
        let mut game: CardRoulette = CardRoulette::new(num_players, num_rounds);
        game.play_game();
        let game_scores: Vec<usize> = game.scores; // Wrap game.scores in a list
        let rounds_dir: String = create_rounds_directory(&players_dir, num_rounds);

        save_game_results(&rounds_dir, game_number as i32, &simulation_id.to_string(), &game_scores);
    }
    
    let csv_file_path: String = format!("{}/{}_players/{}_rounds/{}.csv", manage_results::RESULTS_DIR, num_players, num_rounds, simulation_id);
    let _ = save_accumulated_scores(&csv_file_path, &simulation_id, num_players, num_rounds);
    println!("{}All games played. Results saved.{}\n", GREEN, RESET);
}
