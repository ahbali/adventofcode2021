use std::fs;
fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("could not open file");
    let raw_input = fs::read_to_string("input.txt").expect("could not open file");
    let mut input: Vec<isize> = raw_input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|str| str.parse().ok())
        .collect();
    input.sort();
    let sum = (0..*input.last().unwrap())
        .map(|candidate| {
            input
                .iter()
                .map(|&el| (candidate - el).abs())
                .sum::<isize>()
        })
        .min()
        .unwrap();
    println!("part 1 = {}", sum);

    let sum = (0..*input.last().unwrap())
        .map(|candidate| {
            input
                .iter()
                .map(|&el| ((candidate - el).abs() * ((candidate - el).abs() + 1)) / 2)
                .sum::<isize>()
        })
        .min()
        .unwrap();
    println!("part 2 = {}", sum);
}
