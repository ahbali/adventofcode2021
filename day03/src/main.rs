use std::fs;

// returns tuple (most_common, least_common)
fn find_common(vector: &Vec<usize>, position: usize) -> (Vec<usize>, Vec<usize>) {
    let onces: Vec<usize> = vector
        .iter()
        .filter(|&&value| ((value >> position) & 1 == 1))
        .cloned()
        .collect();
    let zeros: Vec<usize> = vector
        .iter()
        .filter(|&&value| ((value >> position) & 1 == 0))
        .cloned()
        .collect();
    if zeros.len() > onces.len() {
        (zeros, onces)
    } else {
        (onces, zeros)
    }
}

fn oxygen_rating(vector: &Vec<usize>, position: usize) -> usize {
    let (most_common, _) = find_common(vector, position);
    // println!("most common in step {}: {:?}", position, most_common);
    if most_common.len() == 1 {
        return most_common[0];
    }
    oxygen_rating(&most_common, position - 1)
}

fn co2_rating(vector: &Vec<usize>, position: usize) -> usize {
    let (_, least_common) = find_common(vector, position);
    // println!("least common in step {}: {:?}", position, least_common);
    if least_common.len() == 1 {
        return least_common[0];
    }
    co2_rating(&least_common, position - 1)
}

fn main() {
    // let input = fs::read_to_string("sample.txt").expect("error reading file");
    let input = fs::read_to_string("input.txt").expect("error reading file");
    let mut values_number = 0;
    let length = input.lines().take(1).next().unwrap().len();
    //
    // part one
    //
    let mut histogram: Vec<usize> = vec![0; length];
    for line in input.lines() {
        values_number += 1;
        for (pos, chr) in line.chars().enumerate() {
            if chr == '1' {
                histogram[pos] += 1;
            }
        }
    }
    let mut gamma = 0usize;
    for (pos, &value) in histogram.iter().rev().enumerate() {
        if value >= values_number / 2 {
            gamma += 1 << pos;
        }
    }
    let epsilon = (1 << length) - 1 - gamma;
    println!("part1 = {}", gamma * epsilon);

    //
    // part two
    //
    let numbers = input
        .lines()
        .filter_map(|line| usize::from_str_radix(line, 2).ok())
        .collect::<Vec<usize>>();
    let oxygen_rating = oxygen_rating(&numbers, length - 1);
    let co2_rating = co2_rating(&numbers, length - 1);
    println!("part2 = {}", oxygen_rating * co2_rating);
}
