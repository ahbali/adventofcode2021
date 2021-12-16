use std::fs;

const STEPS: usize = 100;

fn index_in_vec((i, j): (usize, usize), cols: usize) -> usize {
    j * cols + i
}

fn index_in_2d_array(idx: usize, cols: usize) -> (usize, usize) {
    (idx % cols, idx / cols)
}

fn neighbors((i0, j0): (usize, usize)) -> [(isize, isize); 8] {
    let mut v: [(isize, isize); 8] = [(0, 0); 8];
    let mut idx = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if (i, j) != (0, 0) {
                v[idx] = (i0 as isize + i, j0 as isize + j);
                idx += 1;
            }
        }
    }
    v
}

fn flash_wave(
    idx: usize,
    cols: usize,
    lines: usize,
    octos_energy: &mut [u8],
    flashed: &mut Vec<usize>,
) -> usize {
    let mut flashes = 1_usize;
    flashed.push(idx);
    octos_energy[idx] = 0;
    for (i, j) in neighbors(index_in_2d_array(idx, cols)) {
        if i >= 0 && j >= 0 && i < cols as isize && j < lines as isize {
            let id = index_in_vec((i as usize, j as usize), cols);
            if !flashed.contains(&id) {
                octos_energy[id] += 1;
                if octos_energy[id] > 9 {
                    flashes += flash_wave(id, cols, lines, octos_energy, flashed);
                }
            }
        }
    }
    flashes
}

fn main() {
    // let raw_input = fs::read_to_string("small_sample.txt").expect("could not read file");
    // let raw_input = fs::read_to_string("sample.txt").expect("could not read file");
    let raw_input = fs::read_to_string("input.txt").expect("could not read file");
    let cols = raw_input.lines().map(|str| str.len()).next().unwrap_or(0);
    let mut octos_energy: Vec<u8> = raw_input
        .chars()
        .filter(|&char| char != '\n')
        .map(|char| char.to_digit(10).unwrap() as u8)
        .collect();
    let lines = octos_energy.len() / cols;
    let mut part1 = 0_usize;

    for _ in 0..STEPS {
        octos_energy = octos_energy.iter().map(|el| el + 1).collect();
        let mut flashed: Vec<usize> = vec![];
        let nines: Vec<usize> = octos_energy
            .iter()
            .enumerate()
            .filter(|&(_, &val)| val > 9)
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();
        for &idx in nines.iter() {
            if !flashed.contains(&idx) {
                part1 += flash_wave(idx, cols, lines, &mut octos_energy, &mut flashed);
            }
        }
        // println!("step {}:", step + 1);
        // for j in 0..lines {
        //     for i in 0..cols {
        //         print!("{} ", octos_energy[index_in_vec((i, j), cols)]);
        //     }
        //     println!();
        // }
    }
    println!("part 1 = {}", part1);
}
