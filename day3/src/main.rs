use std::fs;

fn main() {
    solve1();
    solve2();
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let batteries: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut result = 0;

    for battery in batteries {
        let mut digit1 = battery[0];
        let mut digit2 = battery[1];

        for &digit in &battery[2..] {
            if digit2 > digit1 {
                digit1 = digit2;
                digit2 = digit;
            } else if digit > digit2 {
                digit2 = digit;
            }
        }

        result += digit1 * 10 + digit2;
    }

    println!("Result 1: {}", result);
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let batteries: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut result: i64 = 0;

    for battery in batteries {
        let mut digits = battery[..12].to_vec();

        'outer: for &digit in &battery[12..] {
            let digits_cpy = digits.clone();
            for (i, window) in digits_cpy.windows(2).enumerate() {
                if window[1] > window[0] {
                    digits[i] = digit;
                    digits[i..].rotate_left(1);
                    continue 'outer;
                }
            }
            if digit > digits[11] {
                digits[11] = digit;
            }
        }

        let mut number: i64 = 0;
        for &digit in &digits {
            number = number * 10 + digit as i64;
        }

        result += number;
    }

    println!("Result 2: {}", result);
}
