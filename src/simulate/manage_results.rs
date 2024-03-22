use std::fs;
use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, Read, Write};
use serde_json::{Map, Value};
use std::io::{Lines, Error, Seek};

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub const RESULTS_DIR: &str = "results";

/// Create the 'results' directory if it doesn't exist.
pub fn create_results_directory() {
    if let Err(_) = fs::create_dir(RESULTS_DIR) {
        println!("{}Directory '{}' already exists or creation failed.{}", RED, RESULTS_DIR, RESET);
    }
}

/// Create a directory for the number of players if it doesn't exist.
///
/// # Arguments
///
/// * `num_players` - Number of players in each game.
///
/// # Returns
///
/// * `String` - Path to the directory for the number of players.
pub fn create_player_directory(num_players: usize) -> String {
    let players_dir: String = format!("{}/{}_players", RESULTS_DIR, num_players);

    if let Err(_) = fs::create_dir_all(&players_dir) {
        println!("{}Directory '{}' already exists or creation failed.{}", RED, players_dir, RESET);
    }

    players_dir
}

/// Create a directory for the number of rounds if it doesn't exist.
///
/// # Arguments
///
/// * `players_dir` - Path to the directory for the number of players.
/// * `num_rounds` - Number of rounds in each game.
///
/// # Returns
///
/// * `String` - Path to the directory for the number of rounds.
pub fn create_rounds_directory(players_dir: &str, num_rounds: usize) -> String {
    let rounds_dir: String = format!("{}/{}_rounds", players_dir, num_rounds);

    if let Err(_) = fs::create_dir_all(&rounds_dir) {
        println!("{}Directory '{}' already exists or creation failed.{}", RED, rounds_dir, RESET);
    }

    rounds_dir
}

/// Save the results of a game to a CSV file.
///
/// # Arguments
///
/// * `rounds_dir` - Path to the directory for the number of rounds.
/// * `game_number` - Number of the game.
/// * `simulation_id` - UID of the simulation.
/// * `game_scores` - Scores of each player in the game.
pub fn save_game_results(rounds_dir: &str, game_number: i32, simulation_id: &str, game_scores: &Vec<usize>) {
    let csv_filename: String = format!("{}/{}.csv", rounds_dir, simulation_id);

    let mut file: File = match OpenOptions::new().write(true).create(true).append(true).open(&csv_filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{}Failed to open or create file '{}'{}", RED, csv_filename, RESET);
            return;
        }
    };

    if file.metadata().map(|m| m.len() == 0).unwrap_or(false) {
        if let Err(_) = writeln!(file, "game_number,{}", (1..=game_scores.len()).map(|i| format!("player{}_score", i)).collect::<Vec<_>>().join(",")) {
            eprintln!("{}Failed to write headers to file '{}'{}", RED, csv_filename, RESET);
            return;
        }
    }

    let scores_str: Vec<String> = game_scores.iter().map(|&score| score.to_string()).collect();
    let line: String = format!("{},{}\n", game_number, scores_str.join(","));
    if let Err(_) = file.write_all(line.as_bytes()) {
        eprintln!("{}Failed to write to file '{}'{}", RED, csv_filename, RESET);
    }
}

/// Saves accumulated scores from a CSV file to results/total_results.json.
///
/// # Arguments
///
/// * `csv_file_path` - The path to the CSV file containing the accumulated scores.
/// * `simulation_id` - The ID of the simulation.
/// * `num_players`   - The number of players involved.
/// * `num_rounds`    - The number of rounds played.
///
/// # Errors
///
/// * Failure to open or read the CSV file.
/// * Failure to open, read, or write the JSON file.
/// * Parsing errors while processing CSV data or serializing JSON data.
pub fn save_accumulated_scores(csv_file_path: &str, simulation_id: &str, num_players: usize, num_rounds: usize) -> Result<(), Error> {
    let file: Result<File, Error> = File::open(csv_file_path);

    let file: File = match file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}Failed to open file '{}': {}{}", RED, csv_file_path, e, RESET);
            return Err(e)
        }
    };

    let reader: BufReader<File> = BufReader::new(file);

    let mut accumulated_scores: Vec<usize> = Vec::new();
    for _ in 0..num_players {
        accumulated_scores.push(0);
    }

    let mut lines: Lines<BufReader<File>> = reader.lines();
    lines.next();

    let mut num_games: u64 = 0;

    for line in lines {
        if let Ok(line) = line {
            let scores: Vec<usize> = line
                .split(',')
                .skip(1) // Skip game number
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            // accumulated_scores.extend(scores);
            for (acc_score, &score) in accumulated_scores.iter_mut().zip(scores.iter()) {
                *acc_score += score;
            }
            num_games += 1;
        }
    }

    let mut total_results: Map<String, Value>;
    let json_filename: String = format!("{}/total_results.json", RESULTS_DIR);
    let mut json_file: File = match OpenOptions::new().read(true).write(true).open(&json_filename) {
        Ok(file) => {
            let mut file: File = file;
            let mut json_string: String = String::new();
            file.read_to_string(&mut json_string)?;
            total_results = serde_json::from_str(&json_string)?;
            file
        }
        Err(_) => {
            let file = File::create(&json_filename)?;
            total_results = Map::new();
            file
        }
    };

    let players_key: String = format!("{}_players", num_players);
    let rounds_key: String = format!("{}_rounds", num_rounds);

    let scores_json: Value = serde_json::to_value(&accumulated_scores)?;

    let mut simulation_data: Map<String, Value> = serde_json::Map::new();
    simulation_data.insert("simulation_id".to_owned(), serde_json::Value::String(simulation_id.to_owned()));
    simulation_data.insert("num_games".to_owned(), serde_json::Value::Number(serde_json::Number::from(num_games.to_owned())));
    simulation_data.insert("scores".to_owned(), scores_json);

    let rounds_data: &mut Map<String, Value> = total_results.entry(players_key).or_insert(Value::Object(Map::new())).as_object_mut().unwrap();
    let simulations: &mut Vec<Value> = rounds_data.entry(rounds_key).or_insert(serde_json::Value::Array(vec![])).as_array_mut().unwrap();
    simulations.push(serde_json::Value::Object(simulation_data));

    let updated_json_string = serde_json::to_string_pretty(&total_results);

    json_file.seek(std::io::SeekFrom::Start(0))?; // Move cursor to the beginning of the file
    json_file.set_len(0)?;
    let write_result: Result<(), Error> = json_file.write_all(updated_json_string?.as_bytes());

    // Handle the Result value
    match write_result {
        Ok(_) => {
            // Write successful
            println!("{}JSON data written successfully.{}", GREEN, RESET);
        }
        Err(err) => {
            // Handle the error, for example:
            eprintln!("{}Failed to write JSON data: {}{}", RED, err, RESET);
            // You might want to return or take another action based on this error
        }
    }

    Ok(())
}
