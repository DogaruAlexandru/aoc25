use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn main() {
    solve1(1000);
    solve2();
}

struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

fn solve1(connections_count: u32) {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let junctions: Vec<Junction> = input
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split(',').collect();
            Junction {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            }
        })
        .collect();

    let mut distances: BinaryHeap<Reverse<(OrderedFloat<f64>, (u32, u32))>> = BinaryHeap::new();
    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let dx = junctions[i].x - junctions[j].x;
            let dy = junctions[i].y - junctions[j].y;
            let dz = junctions[i].z - junctions[j].z;
            let dist = ((dx * dx + dy * dy + dz * dz) as f64).sqrt();
            distances.push(Reverse((OrderedFloat(dist), (i as u32, j as u32))));
        }
    }

    let mut connections_made: u32 = 0;
    let mut junction_circuit: HashMap<u32, u32> = HashMap::new(); // junction index to circuit index
    let mut circuit_junctions: HashMap<u32, Vec<u32>> = HashMap::new(); // circuit index to junction indices
    while connections_made < connections_count - 1
        && let Some(Reverse((OrderedFloat(_dist), (i, j)))) = distances.pop()
    {
        let ci = junction_circuit.get(&i);
        let cj = junction_circuit.get(&j);
        match (ci, cj) {
            (None, None) => {
                junction_circuit.insert(i, connections_made);
                junction_circuit.insert(j, connections_made);
                circuit_junctions.insert(connections_made, vec![i, j]);
                connections_made += 1;
            }
            (Some(&ci), None) => {
                junction_circuit.insert(j, ci);
                circuit_junctions.get_mut(&ci).unwrap().push(j);
                connections_made += 1;
            }
            (None, Some(&cj)) => {
                junction_circuit.insert(i, cj);
                circuit_junctions.get_mut(&cj).unwrap().push(i);
                connections_made += 1;
            }
            (Some(&ci), Some(&cj)) => {
                if ci != cj {
                    let junctions_to_move = circuit_junctions.remove(&cj).unwrap();
                    for &junction_index in &junctions_to_move {
                        junction_circuit.insert(junction_index, ci);
                        circuit_junctions.get_mut(&ci).unwrap().push(junction_index);
                    }
                }
                connections_made += 1;
            }
        }
    }

    let mut largest_counts: Vec<usize> = circuit_junctions.values().map(|v| v.len()).collect();
    largest_counts.sort_unstable_by(|a, b| b.cmp(a));
    let result = largest_counts.iter().take(3).product::<usize>();
    println!("Result 1: {}", result);
}

fn solve2() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let junctions: Vec<Junction> = input
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.split(',').collect();
            Junction {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            }
        })
        .collect();

    let mut distances: BinaryHeap<Reverse<(OrderedFloat<f64>, (u32, u32))>> = BinaryHeap::new();
    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let dx = junctions[i].x - junctions[j].x;
            let dy = junctions[i].y - junctions[j].y;
            let dz = junctions[i].z - junctions[j].z;
            let dist = ((dx * dx + dy * dy + dz * dz) as f64).sqrt();
            distances.push(Reverse((OrderedFloat(dist), (i as u32, j as u32))));
        }
    }

    let mut connections_made: u32 = 0;
    let mut junction_circuit: HashMap<u32, u32> = HashMap::new(); // junction index to circuit index
    let mut circuit_junctions: HashMap<u32, Vec<u32>> = HashMap::new(); // circuit index to junction indices
    while let Some(Reverse((OrderedFloat(_dist), (i, j)))) = distances.pop() {
        let ci = junction_circuit.get(&i);
        let cj = junction_circuit.get(&j);
        match (ci, cj) {
            (None, None) => {
                junction_circuit.insert(i, connections_made);
                junction_circuit.insert(j, connections_made);
                circuit_junctions.insert(connections_made, vec![i, j]);
                connections_made += 1;
            }
            (Some(&ci), None) => {
                junction_circuit.insert(j, ci);
                circuit_junctions.get_mut(&ci).unwrap().push(j);
                connections_made += 1;
            }
            (None, Some(&cj)) => {
                junction_circuit.insert(i, cj);
                circuit_junctions.get_mut(&cj).unwrap().push(i);
                connections_made += 1;
            }
            (Some(&ci), Some(&cj)) => {
                if ci != cj {
                    let junctions_to_move = circuit_junctions.remove(&cj).unwrap();
                    for &junction_index in &junctions_to_move {
                        junction_circuit.insert(junction_index, ci);
                        circuit_junctions.get_mut(&ci).unwrap().push(junction_index);
                    }
                }
                connections_made += 1;
            }
        }
        if circuit_junctions.values().next().map_or(0, |v| v.len()) == junctions.len() {
            println!(
                "Result 2: {}",
                junctions[i as usize].x * junctions[j as usize].x
            );

            break;
        }
    }
}
