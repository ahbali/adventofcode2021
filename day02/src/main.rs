use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("error opening file");
    // let input = fs::read_to_string("sample.txt").expect("error opening file");
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for line in input.lines() {
        let mut line_iter = line.split_whitespace();
        let direction = line_iter.next().unwrap();
        let value = line_iter.next().unwrap().parse::<isize>().unwrap();
        match direction {
            "forward" => horizontal_pos += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => unreachable!(),
        }
    }
    println!("part1 result = {}", horizontal_pos * depth);

    horizontal_pos = 0;
    depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        let mut line_iter = line.split_whitespace();
        let direction = line_iter.next().unwrap();
        let value = line_iter.next().unwrap().parse::<isize>().unwrap();
        match direction {
            "forward" => {
                horizontal_pos += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!(),
        }
    }
    println!("part2 result = {}", horizontal_pos * depth);
}
