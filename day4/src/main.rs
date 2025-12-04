use std::fs;

fn main() {
    solve1();
    solve2();
}

const ADJACENT_POS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn is_accessible(mat: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut count = 0;

    for (r, c) in ADJACENT_POS {
        if let Some(adj_row) = mat.get((row as isize + r) as usize) {
            if let Some(adj_char) = adj_row.get((col as isize + c) as usize) {
                if *adj_char == '@' {
                    count += 1;
                    if count > 3 {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let mat: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result = 0;

    for row in 0..mat.len() {
        for col in 0..mat[row].len() {
            let current = mat[row][col];
            if current != '@' {
                continue;
            }
            if is_accessible(&mat, row, col) {
                result += 1;
            }
        }
    }

    println!("Result 1: {}", result);
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let mut mat: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result = 0;
    let mut changed = true;

    while changed {
        let mut new_mat = mat.clone();
        changed = false;

        for row in 0..mat.len() {
            for col in 0..mat[row].len() {
                let current = mat[row][col];
                if current != '@' {
                    continue;
                }
                if is_accessible(&mat, row, col) {
                    result += 1;
                    changed = true;
                    new_mat[row][col] = '.';
                }
            }
        }
        mat = new_mat;
    }

    println!("Result 2: {}", result);
}
