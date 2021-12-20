use std::{collections::HashSet, fs};

#[derive(Debug)]
enum Fold {
    Up(usize),
    Left(usize),
}

fn fold((a, b): (usize, usize), fold: &Fold) -> Option<(usize, usize)> {
    match fold {
        &Fold::Left(axis) if a > axis => Some((2 * axis - a, b)),
        &Fold::Left(axis) if a == axis => None,
        &Fold::Up(axis) if b > axis => Some((a, 2 * axis - b)),
        &Fold::Up(axis) if b == axis => None,
        _ => Some((a, b)),
    }
}

fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("could not open file");
    let raw_input = fs::read_to_string("input.txt").expect("could not open file");
    let mut folds: Vec<Fold> = vec![];
    let mut points: HashSet<(usize, usize)> = HashSet::new();

    for line in raw_input.lines() {
        if line.starts_with(char::is_numeric) {
            let mut line_iter = line.split(',').filter_map(|el| el.parse::<usize>().ok());
            let (a, b) = (line_iter.next().unwrap(), line_iter.next().unwrap());
            points.insert((a, b));
        } else if line.starts_with("fold along x") {
            folds.push(
                line.split('=')
                    .skip(1)
                    .find_map(|el| el.parse::<usize>().ok())
                    .map(|el| Fold::Left(el))
                    .unwrap(),
            );
        } else if line.starts_with("fold along y") {
            folds.push(
                line.split('=')
                    .skip(1)
                    .find_map(|el| el.parse::<usize>().ok())
                    .map(|el| Fold::Up(el))
                    .unwrap(),
            );
        }
    }
    let mut part1: usize = 0;
    for (idx, fld) in folds.iter().enumerate() {
        points = points
            .iter()
            .filter_map(|&point| fold(point, fld))
            .collect();
        if idx == 0 {
            part1 = points.len();
        };
    }
    println!("part1 = {}", part1);

    let max_x = points.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = points.iter().max_by_key(|(_, y)| y).unwrap().1;
    println!("part2 = ");
    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                print!("##");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}
