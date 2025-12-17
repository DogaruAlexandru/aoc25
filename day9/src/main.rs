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
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>();

    let mut edges: Vec<((i32, i32), (i32, i32), (i32, i32), (i32, i32))> = corners
        .iter()
        .zip(corners.iter().cycle().skip(1))
        .zip(corners.iter().cycle().skip(2))
        .zip(corners.iter().cycle().skip(3))
        .take(corners.len())
        .map(|e| {
            let p0 = *(e.0.0.0);
            let p1 = *(e.0.0.1);
            let p2 = *(e.0.1);
            let p3 = *(e.1);
            if p1.0 < p2.0 || (p1.0 == p2.0 && p1.1 < p2.1) {
                (p0, p1, p2, p3)
            } else {
                (p3, p2, p1, p0)
            }
        })
        .collect();

    let outside_col = corners.iter().map(|&(col, _)| col).max().unwrap() + 1;
    let outside_row = corners.iter().map(|&(_, row)| row).max().unwrap() + 1;
    let mut max_rect = 0;
    for i in 0..corners.len() {
        for j in (i + 1)..corners.len() {
            if !is_rect_inside(&mut edges, corners[i], corners[j], outside_col, outside_row) {
                continue;
            }

            let dx = corners[i].1 as i32 - corners[j].1 as i32;
            let dy = corners[i].0 as i32 - corners[j].0 as i32;
            let rect = (dx.abs() as i64 + 1) * (dy.abs() as i64 + 1);
            if rect > max_rect {
                max_rect = rect;
            }
        }
    }

    println!("Result 2: {}", max_rect);
}

fn is_rect_inside(
    edges: &mut Vec<((i32, i32), (i32, i32), (i32, i32), (i32, i32))>,
    corner1: (i32, i32),
    corner2: (i32, i32),
    outside_col: i32,
    outside_row: i32,
) -> bool {
    let (left, right) = if corner1.0 < corner2.0 {
        (corner1.0 + 1, corner2.0 - 1)
    } else {
        (corner2.0 + 1, corner1.0 - 1)
    };
    let (top, bottom) = if corner1.1 < corner2.1 {
        (corner1.1 + 1, corner2.1 - 1)
    } else {
        (corner2.1 + 1, corner1.1 - 1)
    };

    // top and bottom edges
    edges.sort_by(|a, b| a.1.0.cmp(&b.1.0));
    let mut top_count = vec![0];
    let mut bottom_count = vec![0];
    for edge in edges.iter() {
        let p0 = edge.0;
        let p1 = edge.1;
        let p2 = edge.2;
        let p3 = edge.3;

        if p1.0 == p2.0 {
            // vertical edge
            if p1.0 < left {
                continue;
            }

            if p1.1 < top && top < p2.1 {
                //vertical edge intersects top edge
                for c in top_count.iter_mut() {
                    *c += 1;
                }
                if p1.0 < right {
                    top_count.push(0);
                }
            }
            if p1.1 < bottom && bottom < p2.1 {
                //vertical edge intersects bottom edge
                for c in bottom_count.iter_mut() {
                    *c += 1;
                }
                if p1.0 < right {
                    bottom_count.push(0);
                }
            }
        } else {
            // horizontal edge
            if p2.0 < left {
                continue;
            }

            if p1.1 == top {
                //horizontal edge inside top edge
                let e = (outside_col, top);
                let p = (left + 1, top);
                if cross(p, e, p0) * cross(p, e, p3) < 0 {
                    for c in top_count.iter_mut() {
                        *c += 1;
                    }
                    if left < p1.0 && p2.0 < right {
                        top_count.push(0);
                    }
                }
            }
            if p1.1 == bottom {
                //horizontal edge inside bottom edge
                let e = (outside_col, bottom);
                let p = (left + 1, bottom);
                if cross(p, e, p0) * cross(p, e, p3) < 0 {
                    for c in bottom_count.iter_mut() {
                        *c += 1;
                    }
                    if left < p1.0 && p2.0 < right {
                        bottom_count.push(0);
                    }
                }
            }
        }
    }

    if top_count.iter().map(|c| c % 2).any(|c| c != 1)
        || bottom_count.iter().map(|c| c % 2).any(|c| c != 1)
    {
        return false;
    }

    //left and right edges
    edges.sort_by(|a, b| a.1.1.cmp(&b.1.1));
    let mut left_count = vec![0];
    let mut right_count = vec![0];
    for edge in edges.iter() {
        let p0 = edge.0;
        let p1 = edge.1;
        let p2 = edge.2;
        let p3 = edge.3;

        if p1.1 == p2.1 {
            // horizontal edge
            if p1.1 < top {
                continue;
            }

            if p1.0 < left && left < p2.0 {
                //horizontal edge intersects left edge
                for c in left_count.iter_mut() {
                    *c += 1;
                }
                if p1.1 < bottom {
                    left_count.push(0);
                }
            }
            if p1.0 < right && right < p2.0 {
                //horizontal edge intersects right edge
                for c in right_count.iter_mut() {
                    *c += 1;
                }
                if p1.1 < bottom {
                    right_count.push(0);
                }
            }
        } else {
            // vertical edge
            if p2.1 < top {
                continue;
            }

            if p1.0 == left {
                //vertical edge inside left edge
                let e = (left, outside_row);
                let p = (left, top + 1);
                if cross(p, e, p0) * cross(p, e, p3) < 0 {
                    for c in left_count.iter_mut() {
                        *c += 1;
                    }
                    if top < p1.1 && p2.1 < bottom {
                        left_count.push(0);
                    }
                }
            }
            if p1.0 == right {
                //vertical edge inside right edge
                let e = (right, outside_row);
                let p = (right, top + 1);
                if cross(p, e, p0) * cross(p, e, p3) < 0 {
                    for c in right_count.iter_mut() {
                        *c += 1;
                    }
                    if top < p1.1 && p2.1 < bottom {
                        right_count.push(0);
                    }
                }
            }
        }
    }

    if left_count.iter().map(|c| c % 2).any(|c| c != 1)
        || right_count.iter().map(|c| c % 2).any(|c| c != 1)
    {
        return false;
    }

    return true;
}

fn cross(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> i128 {
    return (b.0 as i128 - a.0 as i128) * (c.1 as i128 - a.1 as i128)
        - (b.1 as i128 - a.1 as i128) * (c.0 as i128 - a.0 as i128);
}
