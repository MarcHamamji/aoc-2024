fn main() {
    part1();
    part2();
}

fn part1_count_xmases(grid: &[Vec<char>], start_row: usize, start_col: usize) -> i32 {
    let mut count = 0;

    let height = grid.len();
    let width = grid[0].len();

    for row_offset in -1isize..2 {
        for col_offset in -1isize..2 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let mut valid = true;

            for (i, letter) in "XMAS".chars().enumerate() {
                let row = start_row as isize + i as isize * row_offset;
                let col = start_col as isize + i as isize * col_offset;
                
                if row < 0 || row >= height as isize || col < 0 || col >= width as isize {
                    valid = false;
                    break;
                }

                if grid[row as usize][col as usize] != letter {
                    valid = false;
                    break;
                }
            }

            if valid {
                count += 1;
            }
        }
    }

    count
}

fn part1() {
    println!("=== Part 1 ===");

    let input = std::fs::read_to_string("src/bin/day-04/input.txt").expect("Unable to read file");

    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| {
            l.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().next().unwrap())
                .collect()
        })
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut count = 0;
    for row in 0..height {
        for col in 0..width {
            count += part1_count_xmases(&grid, row, col);
        }
    }

    println!("Number of XMASes: {}", count);
}

fn part2_is_x_mas(grid: &[Vec<char>], start_row: usize, start_col: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    let diagonals = [(1, 1), (1, -1)];


    for (row_offset, col_offset) in diagonals {
        let mut diagonal_valid = false;

        for direction in [1, -1] {
            let row_offset = row_offset * direction;
            let col_offset = col_offset * direction;

            let mut direction_valid = true;

            for (i, letter) in "MAS".chars().enumerate() {
                let row = start_row as isize + (i as isize - 1) * row_offset;
                let col = start_col as isize + (i as isize - 1) * col_offset;

                if row < 0 || row >= height as isize || col < 0 || col >= width as isize {
                    direction_valid = false;
                    break;
                }

                if grid[row as usize][col as usize] != letter {
                    direction_valid = false;
                    break;
                }
            }

            if direction_valid {
                diagonal_valid = true;
                break;
            }
        }

        if !diagonal_valid {
            return false;
        }

    }

    true
}

fn part2() {
    println!("=== Part 2 ===");

    let input = std::fs::read_to_string("src/bin/day-04/input.txt").expect("Unable to read file");

    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| {
            l.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().next().unwrap())
                .collect()
        })
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut count = 0;
    for row in 0..height {
        for col in 0..width {
            if part2_is_x_mas(&grid, row, col) {
                count += 1;
            }
        }
    }

    println!("Number of X-MASes: {}", count);
}
