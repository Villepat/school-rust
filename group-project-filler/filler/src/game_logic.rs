use filler::{anfield::Anfield, piece::Piece};
use std::collections::HashMap;
use std::vec::Vec;

pub struct PotentialSetup {
    pub id: usize,
    pub anfield: Anfield,
    pub x: usize,
    pub y: usize,
    pub distance: usize,
    pub score: f64,
}

// Helper function to find min and max rows where the player's pieces are located
pub fn find_player_rows_min_max(anfield: &Anfield, player: &str) -> (Option<usize>, Option<usize>) {
    let (player_area, player_latest_piece) = if player == "1" {
        ("@", "a")
    } else {
        ("$", "s")
    };

    let mut min_row: Option<usize> = None;
    let mut max_row: Option<usize> = None;

    for i in 0..anfield.grid.len() {
        for j in 0..anfield.grid[0].len() {
            if anfield.grid[i][j] == player_area || anfield.grid[i][j] == player_latest_piece {
                min_row = Some(min_row.map_or(i, |min| std::cmp::min(min, i)));
                max_row = Some(max_row.map_or(i, |max| std::cmp::max(max, i)));
            }
        }
    }

    (min_row, max_row)
}

pub fn possible_moves(
    anfield: &mut Anfield,
    piece: &Piece,
    player: &str,
    distance_cache: &mut HashMap<(usize, usize, String), usize>, // Add this parameter
) -> Vec<PotentialSetup> {
    let opponent_symbols = if player == "1" {
        vec!["s", "$"]
    } else {
        vec!["a", "@"]
    };
    //gets updated when we push a soluction to the return vector
    let mut we_have_a_move = false;
    let opponent_areas = if player == "1" { vec!["s"] } else { vec!["a"] };
    //keeps track of whether the opponent has placed a piece in the last round
    let mut opponent_placed = false;
    //if there is an s or a in the anfield grid, the opponent has placed a piece
    //if not, we can quick fill the map with the first possible solution in the possible_moves function
    for i in 0..anfield.grid.len() {
        for j in 0..anfield.grid[0].len() {
            if opponent_areas.contains(&anfield.grid[i][j].as_str()) {
                opponent_placed = true;
            }
        }
    }
    //initialize min and max rows
    let (min_row, max_row) = find_player_rows_min_max(&anfield, player);
    //initialize amount of rows in piece
    let piece_rows = piece.shape.len();
    //initialize starting row by subtracting piece rows from min row
    let mut starting_row = 0; // Default to 0

    if let Some(min_row_val) = min_row {
        if min_row_val >= piece_rows {
            starting_row = min_row_val - piece_rows;
        } else {
            // Safely default to 0 if piece_rows is greater than min_row_val
            starting_row = 0;
        }
    }

    //initialize ending row as max row
    let ending_row = max_row.unwrap();
    let mut possible_setups: Vec<PotentialSetup> = Vec::new();
    let mut id_counter = 0; // Counter to set unique IDs for each potential setup

    // Symbols for each player's area and latest piece
    // let (player_area, player_latest_piece) = if player == "1" {
    //     ("@", "a")
    // } else {
    //     ("$", "s")
    // };
    ///////////////////////////////////////
    let (player_area, player_latest_piece, opponent_latest_piece) = if player == "1" {
        ("@", "a", "s")
    } else {
        ("$", "s", "a")
    };

    // Loop through each cell in the Anfield grid
    for i in starting_row..ending_row + 1 {
        for j in 0..anfield.grid[0].len() {
            let mut can_place = true;
            let mut overlap_count = 0;

            // Remember what cells were changed
            let mut changed_cells: Vec<(usize, usize, String)> = Vec::new();

            for x in 0..piece.shape.len() {
                for y in 0..piece.shape[0].len() {
                    let anfield_x = i + x;
                    let anfield_y = j + y;

                    if anfield_x >= anfield.grid.len() || anfield_y >= anfield.grid[0].len() {
                        can_place = false;
                        break;
                    }

                    match (
                        piece.shape[x][y].as_str(),
                        anfield.grid[anfield_x][anfield_y].as_str(),
                    ) {
                        ("O", ".") => {
                            // Remember original state
                            changed_cells.push((anfield_x, anfield_y, ".".to_string()));

                            // Modify grid directly
                            anfield.grid[anfield_x][anfield_y] = player_latest_piece.to_string();
                        }
                        ("O", area) if area == player_area || area == player_latest_piece => {
                            overlap_count += 1;
                        }
                        ("O", _) => {
                            can_place = false;
                        }
                        _ => {}
                    }
                }

                if !can_place {
                    break;
                }
            }

            if can_place && overlap_count == 1 {
                id_counter += 1;

                let distance =
                    calculate_distance_to_opponent(&anfield, i, j, player, distance_cache);

                // let squares_claimed = squares_claimed(&anfield, &piece, i, j);
                let squares_blocked = squares_blocked(&anfield, &piece, i, j, &opponent_symbols);
                ////////TESTINGGGGG/////////
                let proximity_to_opponent_latest_piece =
                    distance_to_latest_opponent_piece(&anfield, i, j, opponent_latest_piece);

                // Weights for each factor
                let weight_distance = 1.0;
                // let weight_squares_claimed = 1.5;
                let weight_squares_blocked = 1.2;
                ////////TESTINGGGGG/////////
                let weight_proximity_to_opponent_latest_piece = 1.0;

                // Calculate the score
                let score = weight_distance * (1.0 / (1.0 + distance as f64))
                   // + weight_squares_claimed * squares_claimed as f64
                    + weight_squares_blocked * squares_blocked as f64
                     ////////TESTINGGGGG/////////
                    + weight_proximity_to_opponent_latest_piece * (1.0 / (1.0 + proximity_to_opponent_latest_piece as f64));

                possible_setups.push(PotentialSetup {
                    id: id_counter,
                    anfield: Anfield {
                        grid: anfield.grid.clone(),
                    },
                    x: i,
                    y: j,
                    distance,
                    score, // Include the score
                });
                we_have_a_move = true;
            }
            // Revert the grid back to its original state
            for (x, y, original) in changed_cells {
                anfield.grid[x][y] = original;
            }
        }
        //if opponent has not placed a piece and we have a move, break
        if !opponent_placed && we_have_a_move {
            break;
        }
    }

    // Return the vector of possible setups
    possible_setups
}

pub fn calculate_distance_to_opponent(
    anfield: &Anfield,
    latest_piece_x: usize,
    latest_piece_y: usize,
    player: &str,
    distance_cache: &mut HashMap<(usize, usize, String), usize>,
) -> usize {
    // Check if the result is already in the cache
    if let Some(distance) =
        distance_cache.get(&(latest_piece_x, latest_piece_y, player.to_string()))
    {
        return *distance;
    }
    let opponent_areas = if player == "1" {
        vec!["$", "s"]
    } else {
        vec!["@", "a"]
    };

    let mut min_distance = usize::MAX;

    for i in 0..anfield.grid.len() {
        for j in 0..anfield.grid[0].len() {
            if opponent_areas.contains(&anfield.grid[i][j].as_str()) {
                let distance = (i as isize - latest_piece_x as isize).abs()
                    + (j as isize - latest_piece_y as isize).abs();
                if (distance as usize) < (min_distance as usize) {
                    min_distance = distance as usize;
                }
            }
        }
        //if distance is below 3, break
        if min_distance < 3 {
            break;
        }
    }

    distance_cache.insert(
        (latest_piece_x, latest_piece_y, player.to_string()),
        min_distance,
    );

    min_distance
}

// Calculate the number of squares claimed by placing the piece at (x, y)
// fn squares_claimed(anfield: &Anfield, piece: &Piece, x: usize, y: usize) -> usize {
//     let mut count = 0;
//     for dx in 0..piece.shape.len() {
//         for dy in 0..piece.shape[0].len() {
//             if piece.shape[dx][dy] == "O" && anfield.grid[x + dx][y + dy] == "." {
//                 count += 1;
//             }
//         }
//     }
//     count
// }

// Calculate the number of opponent squares that would be blocked by placing the piece at (x, y)
fn squares_blocked(
    anfield: &Anfield,
    piece: &Piece,
    x: usize,
    y: usize,
    opponents: &[&str],
) -> usize {
    let mut count = 0;
    for dx in 0..piece.shape.len() {
        for dy in 0..piece.shape[0].len() {
            if piece.shape[dx][dy] == "O" {
                let ax = x + dx;
                let ay = y + dy;
                for nx in (ax as isize - 1).max(0) as usize..(ax + 2).min(anfield.grid.len()) {
                    for ny in (ay as isize - 1).max(0) as usize..(ay + 2).min(anfield.grid[0].len())
                    {
                        if opponents.contains(&anfield.grid[nx][ny].as_str()) {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

// Calculate the distance to the opponent's latest piece
fn distance_to_latest_opponent_piece(
    anfield: &Anfield,
    x: usize,
    y: usize,
    opponent_latest_piece: &str,
) -> usize {
    let mut min_distance = usize::MAX;

    for i in 0..anfield.grid.len() {
        for j in 0..anfield.grid[0].len() {
            if anfield.grid[i][j] == opponent_latest_piece {
                let distance = (i as isize - x as isize).abs() + (j as isize - y as isize).abs();
                if distance < min_distance as isize {
                    min_distance = distance as usize;
                }
            }
        }
    }

    min_distance
}
