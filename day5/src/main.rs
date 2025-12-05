use std::fs;

fn main() {
    solve1();
    solve2();
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap().replace("\r\n", "\n");
    let sections: Vec<&str> = input.split("\n\n").collect();
    let ranges: Vec<(i64, i64)> = sections[0]
        .lines()
        .map(|s| {
            let mut parts = s.split('-');
            (
                parts.next().unwrap_or("").parse::<i64>().unwrap_or(0),
                parts.next().unwrap_or("").parse::<i64>().unwrap_or(0),
            )
        })
        .collect();
    let ids: Vec<i64> = sections[1]
        .lines()
        .map(|s| s.parse::<i64>().unwrap_or(0))
        .collect();

    let mut result = 0;
    for id in ids {
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                result += 1;
                break;
            }
        }
    }

    println!("Result 1: {}", result);
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap().replace("\r\n", "\n");
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut ranges: Vec<(i64, i64)> = sections[0]
        .lines()
        .map(|s| {
            let mut parts = s.split('-');
            (
                parts.next().unwrap_or("").parse::<i64>().unwrap_or(0),
                parts.next().unwrap_or("").parse::<i64>().unwrap_or(0),
            )
        })
        .collect();
    ranges.sort_by_key(|k| k.0);

    let mut merged_ranges: Vec<(i64, i64)> = vec![(ranges[0].0, ranges[0].1)];
    'outer: for (start, end) in ranges[1..].iter() {
        for i in 0..merged_ranges.len() {
            if *end <= merged_ranges[i].1 {
                continue 'outer;
            } else if *start <= merged_ranges[i].1 && *end > merged_ranges[i].1 {
                merged_ranges[i].1 = *end;
                continue 'outer;
            }
        }
        merged_ranges.push((*start, *end));
    }

    let mut result = 0;
    for range in &merged_ranges {
        result += range.1 - range.0 + 1;
    }

    println!("Result 2: {}", result);
}
