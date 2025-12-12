use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn main() {
    solve1();
    // solve2();
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let paths: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            (parts[0], parts[1].split_whitespace().collect::<Vec<&str>>())
        })
        .collect();

    let mut result = 0;
    let mut queue = VecDeque::new();
    queue.push_back("you");
    while let Some(current) = queue.pop_front() {
        for next in &paths[current] {
            if *next == "out" {
                result += 1;
            } else {
                queue.push_back(next);
            }
        }
    }

    println!("Result 1: {}", result);
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let paths: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            (parts[0], parts[1].split_whitespace().collect::<Vec<&str>>())
        })
        .collect();

    let mut result = 0;
    let mut visited = HashSet::new();
    visited.insert(("svr", (false, false)));
    let mut queue = VecDeque::new();
    queue.push_back(("svr", (false, false)));

    while let Some(current) = queue.pop_front() {
        for next in &paths[current.0] {
            if *next == "out" {
                if current.1 == (true, true) {
                    result += 1;
                }
            } else {
                if visited.contains(&(*next, current.1)) {
                    continue;
                }
                visited.insert((*next, current.1));
                let (mut dac, mut fft) = current.1;
                match *next {
                    "dac" => dac = true,
                    "fft" => fft = true,
                    _ => {}
                }
                queue.push_back((next, (dac, fft)));
            }
        }
    }

    println!("Result 2: {}", result);
}
