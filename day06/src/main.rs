use std::fs;
fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("could not read file");
    let raw_input = fs::read_to_string("input.txt").expect("could not read file");
    let input = raw_input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|str| str.parse().ok())
        .collect::<Vec<u8>>();

    let mut population = 0usize;
    let mut children_counts: [usize; 7];

    // let mut world: Vec<u8> = Vec::with_capacity(10_000_000_000);
    // world.push(0);

    ////
    //// part 1
    ////
    // children_counts = [0; 7];
    // const PART1_ITERATIONS: usize = 80;

    // for gen in 1..=PART1_ITERATIONS {
    //     for i in 0..world.len() {
    //         if world[i] == 0 {
    //             world[i] = 6;
    //             world.push(8);
    //         } else {
    //             world[i] -= 1;
    //         }
    //     }
    //     if (PART1_ITERATIONS - gen) < 7 {
    //         children_counts[PART1_ITERATIONS - gen] = world.len();
    //     }
    // }
    // println!("children_counts = {:?};", children_counts);

    children_counts = [1421, 1401, 1191, 1154, 1034, 950, 905]; // these values were computed using the above commented code.
    for &element in input.iter() {
        population += children_counts[element as usize];
    }
    println!("part 1= {}", population);
    ////
    //// part 2
    ////
    population = 0;
    // const PART2_ITERATIONS: usize = 256;
    // for gen in (PART1_ITERATIONS + 1)..=PART2_ITERATIONS {
    //     for i in 0..world.len() {
    //         if world[i] == 0 {
    //             world[i] = 6;
    //             world.push(8);
    //         } else {
    //             world[i] -= 1;
    //         }
    //     }
    //     if (PART2_ITERATIONS - gen) < 7 {
    //         children_counts[PART2_ITERATIONS - gen] = world.len();
    //     }
    // }
    // println!("children_counts = {:?};", children_counts);

    children_counts = [
        6703087164, 6206821033, 5617089148, 5217223242, 4726100874, 4368232009, 3989468462,
    ]; // these values were computed using the above commented code.
    for element in input {
        population += children_counts[element as usize];
    }
    println!("part 2= {}", population);
    // let mut iterations = 80;
    // let mut total = iterations / 7;
    // let mut children = iterations / 7;
    // while children != 0 {
    //     for child in 0..children {
    //         total += (iterations -2 - (child * 7)) / 7;
    //     }
    //     iterations = iterations - ((iterations - 2) / 7);
    //     children = (iterations - 2) / 7;
    // }
    // println!("total = {}", total);
}
