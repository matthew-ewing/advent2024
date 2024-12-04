use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn d4() {

    let file_path = "/Users/matthewe/GitHub/advent2024/input/d4.txt";

    let reader = BufReader::new(File::open(file_path).expect("Cannot open file.txt"));
    let mut grid = vec![];

    for line in reader.lines() {
        let char_vec: Vec<char> = line.unwrap().chars().collect();
        grid.push(char_vec);
    }

    let mut results: HashMap<&str, Vec<Vec<(usize, usize)>>> = HashMap::new();
    let mut x_results: HashMap<&str, Vec<Vec<(usize, usize)>>> = HashMap::new();

    results.insert("XMAS", find_all_instances(&grid, "XMAS"));
    x_results.insert("MAS", find_x_shapes(&grid, "MAS"));

    println!("Results: {}; X - Results: {}", results["XMAS"].len(), x_results["MAS"].len());
}

fn find_x_shapes(grid: &[Vec<char>], word: &str) -> Vec<Vec<(usize, usize)>> {
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let half_len = word_len / 2; // Number of characters on either side of the center
    let rows = grid.len();
    let cols = grid[0].len();
    let mut results = vec![];

    for row in 0..rows {
        for col in 0..cols {
            if row >= half_len && row + half_len < rows && col >= half_len && col + half_len < cols {
                let mut is_x_shape = false;
                let mut coordinates = vec![];

                // Check all combinations of directions for the two diagonals
                for &diag1_dir in &[-1, 1] {
                    for &diag2_dir in &[-1, 1] {
                        let mut temp_coordinates = vec![];
                        let mut diag1_match = true;
                        let mut diag2_match = true;

                        // Check the top-left to bottom-right diagonal
                        for i in 0..=half_len {
                            let r1 = row as isize + i as isize * diag1_dir;
                            let c1 = col as isize + i as isize * diag1_dir;
                            let r2 = row as isize - i as isize * diag1_dir;
                            let c2 = col as isize - i as isize * diag1_dir;

                            if r1 < 0 || c1 < 0 || r1 >= rows as isize || c1 >= cols as isize
                                || grid[r1 as usize][c1 as usize] != word_chars[half_len + i]
                                || grid[r2 as usize][c2 as usize] != word_chars[half_len - i]
                            {
                                diag1_match = false;
                                break;
                            }
                            temp_coordinates.push((r1 as usize, c1 as usize));
                            if i > 0 {
                                temp_coordinates.push((r2 as usize, c2 as usize));
                            }
                        }

                        // Check the top-right to bottom-left diagonal
                        for i in 0..=half_len {
                            let r3 = row as isize + i as isize * diag2_dir;
                            let c3 = col as isize - i as isize * diag2_dir;
                            let r4 = row as isize - i as isize * diag2_dir;
                            let c4 = col as isize + i as isize * diag2_dir;

                            if r3 < 0 || c3 < 0 || r3 >= rows as isize || c3 >= cols as isize
                                || grid[r3 as usize][c3 as usize] != word_chars[half_len + i]
                                || grid[r4 as usize][c4 as usize] != word_chars[half_len - i]
                            {
                                diag2_match = false;
                                break;
                            }
                            if !temp_coordinates.contains(&(r3 as usize, c3 as usize)) {
                                temp_coordinates.push((r3 as usize, c3 as usize));
                            }
                            if i > 0 && !temp_coordinates.contains(&(r4 as usize, c4 as usize)) {
                                temp_coordinates.push((r4 as usize, c4 as usize));
                            }
                        }

                        if diag1_match && diag2_match {
                            is_x_shape = true;
                            coordinates = temp_coordinates.clone();
                        }
                    }
                }

                if is_x_shape {
                    results.push(coordinates);
                }
            }
        }
    }
    results
}

fn find_all_instances(grid: &[Vec<char>], word: &str) -> Vec<Vec<(usize, usize)>> {
    let directions = vec![
        (-1, 0), (1, 0), // up, down
        (0, -1), (0, 1), // left, right
        (-1, -1), (-1, 1), (1, -1), (1, 1), // diagonals
    ];
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut results = vec![];

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == word_chars[0] {
                for &(dx, dy) in &directions {
                    let mut path = vec![(row, col)];
                    let mut found = true;

                    for k in 1..word_chars.len() {
                        let new_row = row as isize + k as isize * dx;
                        let new_col = col as isize + k as isize * dy;

                        if new_row < 0 || new_col < 0 || 
                           new_row >= rows as isize || new_col >= cols as isize ||
                           grid[new_row as usize][new_col as usize] != word_chars[k] {
                            found = false;
                            break;
                        }
                        path.push((new_row as usize, new_col as usize));
                    }

                    if found {
                        results.push(path);
                    }
                }
            }
        }
    }
    results
}
