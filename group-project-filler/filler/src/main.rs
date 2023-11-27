use filler::{anfield::Anfield, piece::Piece, utils::io::read_from_stdin};

pub mod game_logic;

use game_logic::possible_moves;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Instant;

fn write_to_file(filename: &str, data: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;
    file.write_all(data.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn main() {
    let mut playernumber: Option<usize> = None;
    let mut distance_cache: HashMap<(usize, usize, String), usize> = HashMap::new(); // Declare the cache here

    let mut total_time = 0u128; // Total time for all calculations
    let mut move_count = 0; // Number of moves processed

    loop {
        let start_time = Instant::now(); // Start the timer

        match read_from_stdin() {
            Ok((piece_data, anfield_data, playernr)) => {
                let mut anfield = Anfield { grid: anfield_data };
                let piece = Piece { shape: piece_data };

                if playernumber.is_none() {
                    playernumber = Some(playernr);
                }

                let playernr_str = playernumber.unwrap().to_string();

                let possible_setups =
                    possible_moves(&mut anfield, &piece, &playernr_str, &mut distance_cache);

                let mut max_score = f64::MIN;
                let mut best_move_x = 0;
                let mut best_move_y = 0;
                for setup in possible_setups {
                    if setup.score > max_score {
                        max_score = setup.score;
                        best_move_x = setup.x;
                        best_move_y = setup.y;
                    }
                }

                println!("{} {}", best_move_y, best_move_x);
            }
            Err(error) => {
                eprintln!("Could not read from stdin: {}", error);
                std::process::exit(1);
            }
        }

        let elapsed_time = start_time.elapsed().as_micros(); // Stop the timer
        total_time += elapsed_time;
        move_count += 1;

        // Every 10th move, compute and write the average time for all moves to a file
        if move_count % 10 == 0 {
            let avg_time = total_time / move_count as u128; // Average time for all moves
            let data = format!(
                "Average time for all moves: {} microseconds, Moves made: {}\n",
                avg_time, move_count
            );
            write_to_file(
                "average_times_cached_distances_more_min_distance.txt",
                &data,
            )
            .unwrap();
        }
    }
}
