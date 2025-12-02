use std::fs;

fn main() {
    solve1();
    solve2();
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut result = 0;
    let mut pos = 50;
    for line in &lines {
        let (dir, dist) = line
            .chars()
            .next()
            .map(|c| {
                let dist = line[1..].parse::<i32>().unwrap();
                (c, dist)
            })
            .unwrap();

        pos = match dir {
            'R' => pos + dist,
            'L' => pos - dist,
            _ => pos,
        };
        pos = pos.rem_euclid(100);

        if pos == 0 {
            result += 1;
        }
    }

    println!("Result 1: {}", result);
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut result = 0;
    let mut pos = 50;
    for line in &lines {
        let (dir, dist) = line
            .chars()
            .next()
            .map(|c| {
                let dist = line[1..].parse::<i32>().unwrap();
                (c, dist)
            })
            .unwrap();

        let new_pos = match dir {
            'R' => pos + dist,
            'L' => pos - dist,
            _ => pos,
        };
        let jumps = new_pos / 100;

        if new_pos < 0 {
            result += -jumps;
            if pos != 0 {
                result += 1;
            }
        } else if new_pos > 0 {
            result += jumps;
        } else {
            result += 1
        }
        pos = new_pos.rem_euclid(100);
    }

    println!("Result 2: {}", result);
}
