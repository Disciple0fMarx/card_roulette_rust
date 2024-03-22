mod card_roulette;
mod simulate;

use std::env;
use std::process;
use std::time::{Instant, Duration};

use card_roulette::CardRoulette;
use simulate::simulate_games;

const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

// enum Command {
//     Play,
//     Simulate,
//     Help
// }

fn print_help() {
    println!("{}Card Roulette CLI{}", YELLOW, RESET);
    println!("{}Usage: card_roulette <command> [<args>]{}", YELLOW, RESET);
    println!();
    println!("{}Available commands:{}", YELLOW, RESET);
    println!("{}  play           Start a card roulette game{}", YELLOW, RESET);
    println!("{}  simulate       Simulate multiple card roulette games{}", YELLOW, RESET);
    println!("{}  help           Show help message{}", YELLOW, RESET);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}Error: No command specified.{}", RED, RESET);
        print_help();
        process::exit(1);
    }

    match args[1].as_str() {
        "play" => {
            let start_time: Instant = Instant::now();

            let num_players: usize;
            let num_rounds: usize;
            
            if args.len() == 6 {
                num_players = args[3].parse().expect(&format!("{}Invalid number of players{}", RED, RESET));
                num_rounds = args[5].parse().expect(&format!("{}Invalid number of rounds{}", RED, RESET));
            } else {
                num_players = 3;
                num_rounds = 3;
            }

            let mut game: CardRoulette = CardRoulette::new(num_players, num_rounds);
            game.play_game();

            let end_time: Instant = Instant::now();
            let elapsed_time: Duration = end_time.duration_since(start_time);
            println!("{}Elapsed time: {:?}{}", BLUE, elapsed_time, RESET);
        }
        "simulate" => {
            let start_time: Instant = Instant::now();

            let num_games: usize;
            let num_players: usize;
            let num_rounds: usize;
            
            if args.len() == 8 {
                num_games = args[3].parse().expect(&format!("{}Invalid number of games{}", RED, RESET));
                num_players = args[5].parse().expect(&format!("{}Invalid number of players{}", RED, RESET));
                num_rounds = args[7].parse().expect(&format!("{}Invalid number of rounds{}", RED, RESET));
            } else {
                num_games = 5;
                num_players = 3;
                num_rounds = 3;
            }

            simulate_games(num_games, num_players, num_rounds);

            let end_time: Instant = Instant::now();
            let elapsed_time: Duration = end_time.duration_since(start_time);
            println!("{}Elapsed time: {:?}{}", BLUE, elapsed_time, RESET);
        }
        "help" => {
            if args.len() == 3 {
                match args[2].as_str() {
                    "play" => {
                        println!("{}Usage: card_roulette play --players <num_players> --rounds <num_rounds>{}", YELLOW, RESET);
                    }
                    "simulate" => {
                        println!("{}Usage: card_roulette simulate --games <num_games> --players <num_players> --rounds <num_rounds>{}", YELLOW, RESET);
                    }
                    _ => {
                        println!("{}Unknown command '{}'{}", RED, args[2], RESET);
                        process::exit(1);
                    }
                }
            } else {
                print_help();
            }
        }
        _ => {
            println!("{}Unknown command '{}'{}", RED, args[1], RESET);
            process::exit(1);
        }
    }
}
