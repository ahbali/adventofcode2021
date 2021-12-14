use std::fs;

fn index_in_vec((i, j): (usize, usize), length: usize) -> usize {
    j * length + i
}

fn index_in_2d_array(idx: usize, length: usize) -> (usize, usize) {
    (idx % length, idx / length)
}

fn neighbors((i, j): (usize, usize)) -> [(isize, isize); 4] {
    [
        (i as isize + 1, j as isize),
        (i as isize - 1, j as isize),
        (i as isize, j as isize + 1),
        (i as isize, j as isize - 1),
    ]
}

fn basin_size(
    (i0, j0): (usize, usize),
    map: &Vec<u8>,
    visited: &mut Vec<usize>,
    length: usize,
) -> usize {
    let idx = index_in_vec((i0, j0), length);
    if visited.contains(&idx) {
        return 0;
    }
    visited.push(idx);
    let mut size = 1;
    for (i, j) in neighbors((i0, j0)) {
        if i >= 0 && j >= 0 && i < length as isize && j < (map.len() / length) as isize {
            let val = map[index_in_vec((i as usize, j as usize), length)];
            if val < 9 && val > map[idx] {
                size += basin_size((i as usize, j as usize), map, visited, length);
            }
        }
    }
    return size;
}

fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("could not read file");
    let raw_input = fs::read_to_string("input.txt").expect("could not read file");
    let length = raw_input.lines().map(|str| str.len()).next().unwrap_or(0);
    let input: Vec<u8> = raw_input
        .chars()
        .filter(|&char| char != '\n')
        .map(|char| char.to_digit(10).unwrap() as u8)
        .collect();
    let lines = input.len() / length;

    let mut part1: usize = 0;
    let mut visited: Vec<usize> = vec![];
    let mut basin_sizes: Vec<usize> = vec![];
    for (idx, &current_val) in input.iter().enumerate() {
        let (i, j) = index_in_2d_array(idx, length);
        let lower_neighbors = neighbors((i, j))
            .iter()
            .filter(|(a, b)| a >= &0 && b >= &0 && a < &(length as isize) && b < &(lines as isize))
            .map(|&(a, b)| (a as usize, b as usize))
            .map(|point| input[index_in_vec(point, length)])
            .filter(|&value| value <= current_val)
            .count();
        if lower_neighbors == 0 {
            part1 += (current_val + 1) as usize;
            basin_sizes.push(basin_size((i, j), &input, &mut visited, length));
            visited.clear();
        }
    }
    println!("part 1 = {}", part1);
    basin_sizes.sort_unstable();
    let part2: usize = basin_sizes.iter().rev().take(3).product();
    println!("part 2 = {}", part2);
}
