use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // let input = fs::read_to_string("test_input.txt")?;
    let input = fs::read_to_string("input.txt")?;
    let numbers: Vec<usize> = input
        .lines()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect();

    let part_one = numbers.windows(2).filter(|win| win[1] > win[0]).count();
    println!("part1={}", part_one);

    let part_two = numbers
        .windows(3)
        .map(|slice| slice.iter().sum::<usize>())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|win| win[1] > win[0])
        .count();
    println!("part2={}", part_two);

    Ok(())
}
