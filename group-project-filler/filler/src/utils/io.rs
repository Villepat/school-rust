use std::io;
pub type Dimensions = (isize, isize);

pub fn read_from_stdin() -> Result<(Vec<Vec<String>>, Vec<Vec<String>>, usize), String> {
    let mut piece_data: Vec<Vec<String>> = Vec::new();
    let mut anfield_data: Vec<Vec<String>> = Vec::new();
    let mut playernr: usize = 0;
    let mut read_anfield = false;
    let mut read_piece = false;
    let mut remaining_piece_lines: usize = 0;
    let mut row_counter = 0;
    let mut done_reading = false;

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let line = input.trim().to_string(); // Trim the input line

                if line.contains("$$$ exec p") {
                    playernr = player(&line);
                }

                if line.starts_with("Piece ") {
                    read_anfield = false;
                    read_piece = true;
                    remaining_piece_lines = dimensions(&line).1 as usize;

                    remaining_piece_lines += 1;
                }

                if line.starts_with("Anfield ") {
                    read_anfield = true;
                    read_piece = false;
                }

                if read_anfield {
                    row_counter += 1;
                    if row_counter > 2 {
                        let row: Vec<String> = line[4..].chars().map(|c| c.to_string()).collect();

                        anfield_data.push(row);
                    }
                }

                if read_piece && remaining_piece_lines > 0 {
                    let row: Vec<String> = line.chars().map(|c| c.to_string()).collect();
                    piece_data.push(row);
                    remaining_piece_lines -= 1;

                    // Only break out of the loop when you're sure you've read all lines
                    if remaining_piece_lines == 0 {
                        read_piece = false; // Reset the flag
                        done_reading = true;
                    }
                }

                if done_reading {
                    break;
                }
            }

            Err(e) => return Err(e.to_string()),
        }
    }

    // Remove the first row
    if !piece_data.is_empty() {
        piece_data.remove(0);
    }

    Ok((piece_data, anfield_data, playernr))
}

// Existing player function
pub fn player(line: &str) -> usize {
    match line.split_ascii_whitespace().collect::<Vec<&str>>()[2] {
        "p1" => 1,
        _ => 2,
    }
}

// Existing dimensions function
pub fn dimensions(s: &str) -> Dimensions {
    let dimensions: Vec<&str> = s
        .trim_end()
        .trim_end_matches(':')
        .split_ascii_whitespace()
        .skip(1)
        .collect();

    (
        dimensions[0].parse().unwrap(),
        dimensions[1].parse().unwrap(),
    )
}
