use std::fs;

fn main() {
    let (shapes, regions) = read_input();
    solve1(&shapes, &regions);
    // solve2(&shapes, &regions);
}

fn read_input() -> (Vec<Vec<Vec<char>>>, Vec<((usize, usize), Vec<usize>)>) {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let data = input
        .split("\r\n\r\n")
        .map(|s| s.lines().collect())
        .collect::<Vec<Vec<&str>>>();

    let shapes = data[..data.len() - 1]
        .iter()
        .map(|block| {
            block[1..]
                .iter()
                .map(|&line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    let regions = data
        .last()
        .unwrap()
        .iter()
        .map(|&line| {
            let parts = line.split(':').collect::<Vec<&str>>();

            let sizes = parts[0].split('x').collect::<Vec<&str>>();
            let (cols, rows) = (
                sizes[0].parse::<usize>().unwrap(),
                sizes[1].parse::<usize>().unwrap(),
            );
            let presents = parts[1]
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            ((cols, rows), presents)
        })
        .collect::<Vec<((usize, usize), Vec<usize>)>>();

    return (shapes, regions);
}

fn solve1(shapes: &Vec<Vec<Vec<char>>>, regions: &Vec<((usize, usize), Vec<usize>)>) {
    let spaces = shapes
        .iter()
        .map(|shape| {
            shape
                .iter()
                .map(|row| row.iter().filter(|&&c| c == '#').count())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    let mut result = 0;
    for ((cols, rows), presents) in regions.iter() {
        let mut area = 0;
        for (index, count) in presents.iter().enumerate() {
            area += spaces[index] * count;
        }
        if area <= cols * rows {
            result += 1;
        }
    }

    println!("{}", result);
}

// fn solve2(shapes: &Vec<Vec<Vec<char>>>, regions: &Vec<((usize, usize), Vec<usize>)>) {}
