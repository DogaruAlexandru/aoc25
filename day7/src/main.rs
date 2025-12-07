use std::fs;

fn main() {
    solve1();
    solve2();
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut beams = vec!['.'; matrix[0].len()];
    let mut splits = 0;

    for (index, line) in matrix.iter().enumerate() {
        if index % 2 != 0 {
            continue;
        }
        for (i, &ch) in line.iter().enumerate() {
            match ch {
                'S' => beams[i] = '|',
                '^' => {
                    if beams[i] == '|' {
                        splits += 1;
                        if let Some(left) = beams.get_mut(i - 1) {
                            *left = '|';
                        }
                        beams[i] = '.';
                        if let Some(right) = beams.get_mut(i + 1) {
                            *right = '|';
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    println!("Result 1: {}", splits);
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut beams: Vec<u64> = vec![0; matrix[0].len()];

    for (index, line) in matrix.iter().enumerate() {
        if index % 2 != 0 {
            continue;
        }
        for (i, &ch) in line.iter().enumerate() {
            match ch {
                'S' => beams[i] = 1,
                '^' => {
                    if beams[i] != 0 {
                        if i > 0 {
                            beams[i - 1] += beams[i];
                        }
                        if i < beams.len() - 1 {
                            beams[i + 1] += beams[i];
                        }
                        beams[i] = 0;
                    }
                }
                _ => continue,
            }
        }
    }

    println!("Result 2: {}", beams.iter().sum::<u64>());
}
