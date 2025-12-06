use std::fs;

fn main() {
    solve1();
    solve2();
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|s| s.trim().split_whitespace().collect())
        .collect();
    let mat: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|row| {
            row.iter()
                .map(|num_str| num_str.parse::<i64>().unwrap_or(0))
                .collect()
        })
        .collect();
    let symbols: Vec<&str> = lines[lines.len() - 1].clone();

    let mut result = 0;
    for col in 0..mat[0].len() {
        if symbols[col] == "*" {
            result += mat.iter().map(|row| row[col]).product::<i64>();
        } else if symbols[col] == "+" {
            result += mat.iter().map(|row| row[col]).sum::<i64>();
        }
    }

    println!("Result 1: {}", result);
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let symbols: Vec<char> = lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();
    let matrix: Vec<Vec<char>> = lines[..lines.len() - 1]
        .iter()
        .map(|s| s.chars().collect())
        .collect();

    let mut symbol_index = 0;
    let mut result: i64 = 0;
    let mut aux_result: i64 = match symbols[0] {
        '+' => 0,
        '*' => 1,
        _ => 0,
    };
    for col in 0..matrix[0].len() {
        let mut number: i64 = 0;
        for row in 0..matrix.len() {
            if matrix[row][col] == ' ' {
                continue;
            }
            number = number * 10 + matrix[row][col].to_digit(10).unwrap() as i64;
        }

        if number == 0 {
            symbol_index += 1;
            result += aux_result;
            aux_result = match symbols[symbol_index] {
                '+' => 0,
                '*' => 1,
                _ => 0,
            };
            continue;
        }

        if symbols[symbol_index] == '+' {
            aux_result += number;
        } else if symbols[symbol_index] == '*' {
            aux_result *= number;
        }
    }
    result += aux_result;

    println!("Result 2: {}", result);
}
