use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    solve1();
    solve2();
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

    let mut cache = HashMap::new();
    cache.insert(("out".to_string(), true, true), 1);
    cache.insert(("out".to_string(), true, false), 0);
    cache.insert(("out".to_string(), false, true), 0);
    cache.insert(("out".to_string(), false, false), 0);
    search(&paths, &mut cache, "svr", false, false);

    println!("Result 2: {}", cache[&("svr".to_string(), false, false)]);
}

fn search(
    paths: &HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<(String, bool, bool), usize>,
    current: &str,
    dac: bool,
    fft: bool,
) {
    if cache.contains_key(&(current.to_string(), dac, fft)) {
        return;
    }

    for next in &paths[current] {
        let (mut new_dac, mut new_fft) = (dac, fft);
        match *next {
            "dac" => new_dac = true,
            "fft" => new_fft = true,
            _ => {}
        }
        if !cache.contains_key(&((*next).to_string(), new_dac, new_fft)) {
            search(paths, cache, next, new_dac, new_fft);
        }
        let count = cache[&((*next).to_string(), new_dac, new_fft)];
        *cache.entry((current.to_string(), dac, fft)).or_insert(0) += count;
    }
}
