use std::fs;

fn index_in_vec((i, j): (usize, usize), length: usize) -> usize {
    j * length + i
}
fn index_in_2d_array(idx: usize, length: usize) -> (usize, usize) {
    (idx % length, idx / length)
}
fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("could not read file");
    let raw_input = fs::read_to_string("input.txt").expect("could not read file");
    let length = raw_input.lines().map(|str| str.len()).next().unwrap_or(0);
    let input: Vec<u8> = raw_input
        .chars()
        .filter(|&char| char != '\n')
        .filter_map(|char| char.to_digit(10))
        .filter_map(|num| num.try_into().ok())
        .collect();
    let lines = input.len() / length;

    let mut part1: usize = 0;
    for (idx, &current_val) in input.iter().enumerate() {
        let (i, j) = index_in_2d_array(idx, length);
        let neighbors = vec![
            (i as isize + 1, j as isize),
            (i as isize - 1, j as isize),
            (i as isize, j as isize + 1),
            (i as isize, j as isize - 1),
        ];
        let lower_neighbors = neighbors
            .iter()
            .filter(|(a, b)| a >= &0 && b >= &0 && a < &(length as isize) && b < &(lines as isize))
            .map(|&(a, b)| (a as usize, b as usize))
            .map(|point| input[index_in_vec(point, length)])
            .filter(|&value| value <= current_val)
            .count();
        // println!("neighbors count = {}", lower_neighbors);
        if lower_neighbors == 0 {
            part1 += (current_val + 1) as usize;
        }
    }
    println!("part 1 = {}", part1);
}
