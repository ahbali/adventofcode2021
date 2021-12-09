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

    //
    // part 1
    //
    let mut world: Vec<u8> = Vec::with_capacity(1_000_000_000);
    for element in input {
        world.push(element);
        for _ in 0..80 {
            for i in 0..world.len() {
                if world[i] == 0 {
                    world[i] = 6;
                    world.push(8);
                } else {
                    world[i] -= 1;
                }
            }
        }
        population += world.len();
        world.clear();
    }
    println!("part 1= {}", population);
}
