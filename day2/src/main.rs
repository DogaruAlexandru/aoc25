use std::fs;

fn main() {
    solve1();
    solve2();
}

fn digit_count(n: u64) -> u32 {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as u32 + 1
    }
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let ranges: Vec<(&str, &str)> = input
        .split(',')
        .map(|s| {
            let mut parts = s.split('-');
            (parts.next().unwrap_or(""), parts.next().unwrap_or(""))
        })
        .collect();

    let mut result = 0;
    for (start, end) in ranges {
        let start_num = start.parse().unwrap_or(0);
        let end_num = end.parse().unwrap_or(0);
        let mut count = digit_count(start_num);
        let mut curr: u64 = match count % 2 {
            0 => start_num / 10u64.pow(count / 2),
            _ => 10u64.pow(count / 2),
        };

        loop {
            count = digit_count(curr);
            let number = curr * 10u64.pow(count) + curr;
            match number {
                n if n < start_num => curr += 1,
                n if n > end_num => break,
                _ => {
                    curr += 1;
                    result += number;
                }
            }
        }
    }

    println!("Result 1: {}", result);
}

fn check_number(num: u64) -> bool {
    let chars = num.to_string().chars().collect::<Vec<char>>();
    let size = chars.len();
    let mut steps = 1;

    'outer: while steps <= size / 2 {
        if size % steps != 0 {
            steps += 1;
            continue;
        }

        let reps = size / steps;
        for step in 0..steps {
            let ch = chars[step];
            for rep in 1..reps {
                if chars[step + rep * steps] != ch {
                    steps += 1;
                    continue 'outer;
                }
            }
        }
        return true;
    }

    return false;
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let ranges: Vec<(&str, &str)> = input
        .split(',')
        .map(|s| {
            let mut parts = s.split('-');
            (parts.next().unwrap_or(""), parts.next().unwrap_or(""))
        })
        .collect();

    let mut result = 0;
    for (start, end) in ranges {
        let start_num = start.parse().unwrap_or(0);
        let end_num = end.parse().unwrap_or(0);
        for num in start_num..=end_num {
            if check_number(num) {
                result += num;
            }
        }
    }

    println!("Result 2: {}", result);
}
