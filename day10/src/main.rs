use regex::Regex;
use std::{fs, vec};
use z3::Optimize;
use z3::ast::Int;

fn main() {
    solve1();
    solve2();
}

fn solve1() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let re_parts =
        Regex::new(r"(?:\[(.+)\])\s((?:\(\d+(?:\,\d+)*\)\s?)+)\s\{(\d+(?:\,\d+)*)\}").unwrap();
    let re_wiring = Regex::new(r"\(((?:\d+\,*)+)\)").unwrap();

    let mut data = vec![];
    for line in input.lines() {
        if let Some(caps) = re_parts.captures(line) {
            let indicator = caps[1]
                .chars()
                .map(|c| if c == '.' { false } else { true })
                .collect::<Vec<bool>>();

            let wiring = re_wiring
                .captures_iter(&caps[2])
                .map(|wcaps| {
                    wcaps[1]
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            data.push((indicator, wiring));
        }
    }

    let mut result = 0;
    for (indicator, wiring) in data.iter() {
        let mut presses = 0;
        let mut vec_from = vec![indicator.clone()];
        let mut vec_to = vec![];

        'outer: while vec_from.len() > 0 {
            for state in vec_from.iter() {
                for i in presses..wiring.len() {
                    let pressed = press_buttons(&state, &wiring[i]);
                    if pressed.iter().all(|&b| b == false) {
                        result += presses + 1;
                        break 'outer;
                    } else {
                        vec_to.push(pressed);
                    }
                }
            }
            std::mem::swap(&mut vec_from, &mut vec_to);
            vec_to.clear();
            presses += 1;
        }
    }

    println!("Result 1: {}", result);
}

fn press_buttons(state: &Vec<bool>, buttons: &Vec<usize>) -> Vec<bool> {
    let mut pressed = state.clone();
    for b in buttons.iter() {
        pressed[*b] = !pressed[*b];
    }
    return pressed;
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let re_parts =
        Regex::new(r"(?:\[(.+)\])\s((?:\(\d+(?:\,\d+)*\)\s?)+)\s\{(\d+(?:\,\d+)*)\}").unwrap();
    let re_wiring = Regex::new(r"\(((?:\d+\,*)+)\)").unwrap();

    let mut data = vec![];
    for line in input.lines() {
        if let Some(caps) = re_parts.captures(line) {
            let wiring = re_wiring
                .captures_iter(&caps[2])
                .map(|wcaps| {
                    wcaps[1]
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            let joltage = caps[3]
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            data.push((wiring, joltage));
        }
    }

    let mut result = 0;
    for (wiring, joltage) in data.iter() {
        let optimzer = Optimize::new();
        let mut coefficients = vec![];
        let mut sum = Int::from_u64(0);
        let mut aux: Vec<Int> = joltage.iter().map(|j| Int::from_u64(*j as u64)).collect();

        for i in 0..wiring.len() {
            let name = format!("c{}", i);
            let c = Int::fresh_const(&name);

            sum += &c;
            optimzer.assert(&c.ge(&Int::from_u64(0)));

            for w in wiring[i].iter() {
                aux[*w] -= &c;
            }

            coefficients.push(c);
        }

        for a in aux.iter() {
            optimzer.assert(&a.eq(&Int::from_u64(0)));
        }
        optimzer.minimize(&sum);

        if optimzer.check(&[]) == z3::SatResult::Sat {
            let model = optimzer.get_model().unwrap();
            for c in coefficients.iter() {
                let val = model.eval(c, true).unwrap().as_u64().unwrap();
                result += val;
            }
        }
    }

    println!("Result 2: {}", result);
}
