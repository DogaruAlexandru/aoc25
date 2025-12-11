use std::fs;

fn main() {
    solve1();
    solve2();
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let corners = input
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split(',').collect();
            (
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<(i64, i64)>>();

    let mut max_rect = 0;
    for i in 0..corners.len() {
        for j in (i + 1)..corners.len() {
            let dx = corners[i].1 - corners[j].1;
            let dy = corners[i].0 - corners[j].0;
            let rect = (dx.abs() + 1) * (dy.abs() + 1);
            if rect > max_rect {
                max_rect = rect;
            }
        }
    }
    println!("Result 1: {}", max_rect);
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let corners = input
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split(',').collect();
            (
                parts[0].parse::<usize>().unwrap(),
                parts[1].parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    let mut horizontal_lines = vec![];
    let mut vertical_lines = vec![];
    let mut push_line = |pos1: (usize, usize), pos2: (usize, usize)| {
        if pos1.0 == pos2.0 {
            vertical_lines.push(((pos1.0, pos1.1.min(pos2.1)), (pos1.0, pos1.1.max(pos2.1))));
        } else {
            horizontal_lines.push(((pos1.0.min(pos2.0), pos1.1), (pos1.0.max(pos2.0), pos1.1)));
        }
    };
    for (pos1, pos2) in corners.windows(2).map(|w| (w[0], w[1])) {
        push_line(pos1, pos2);
    }
    if let (Some(first), Some(last)) = (corners.first(), corners.last()) {
        push_line(*last, *first);
    }

    // let outside_col = corners.iter().map(|&(col, _)| col).max().unwrap() + 1;
    // let outside_row = corners.iter().map(|&(_, row)| row).max().unwrap() + 1;
    // let mut max_rect = 0;
    // for i in 0..corners.len() {
    //     for j in (i + 1)..corners.len() {
    //         if !is_rect(&matrix, corners[i], corners[j]) {
    //             continue;
    //         }

    //         let dx = corners[i].1 as i32 - corners[j].1 as i32;
    //         let dy = corners[i].0 as i32 - corners[j].0 as i32;
    //         let rect = (dx.abs() + 1) * (dy.abs() + 1);
    //         if rect > max_rect {
    //             max_rect = rect;
    //         }
    //     }
    // }

    // println!("Result 2: {}", max_rect);
}
