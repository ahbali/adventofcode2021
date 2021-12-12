use std::fs;

fn quantify(ch: char) -> usize {
    match ch {
        'a' => 1 << 0,
        'b' => 1 << 1,
        'c' => 1 << 2,
        'd' => 1 << 3,
        'e' => 1 << 4,
        'f' => 1 << 5,
        'g' => 1 << 6,
        _ => unreachable!(),
    }
}

fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("could not read file");
    let raw_input = fs::read_to_string("input.txt").expect("could not read file");
    let part1 = raw_input
        .lines()
        .map(|str| str.split('|').skip(1).next().unwrap())
        .flat_map(|str| str.split_whitespace())
        .map(|str| str.len())
        .filter(|&len| (2..=4).contains(&len) || len == 7)
        .count();
    println!("part 1 = {}", part1);
    // chars   numbers
    // 2  ->  1,
    // 3  ->  7,
    // 4  ->  4,
    // 5  ->  2, 3, 5,
    // 6  ->  0, 6, 9,
    // 7  ->  8,
    // =====================
    // 4 | 2 = 8 => 2
    // 1 | 6 = 8 => 6
    // 4 | 9 = 9 => 0, 9
    // 1 | 3 = 3 => 3, 5
    //
    let mut part2 = 0_usize;
    for line in raw_input.lines() {
        let mut line_iter = line.split('|');
        let quantified_patterns: Vec<(usize, usize)> = line_iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|str| {
                (
                    str.len(),
                    str.chars().map(|char| quantify(char)).sum::<usize>(),
                )
            })
            .collect();
        let mut numbers = [&0usize; 10];
        let mut fives: Vec<&usize> = vec![];
        let mut sixes: Vec<&usize> = vec![];
        for (lit_segments, quant_val) in quantified_patterns.iter() {
            match lit_segments {
                2 => numbers[1] = quant_val,
                3 => numbers[7] = quant_val,
                4 => numbers[4] = quant_val,
                5 => fives.push(quant_val),
                6 => sixes.push(quant_val),
                7 => numbers[8] = quant_val,
                _ => unreachable!(),
            }
        }
        numbers[2] = fives
            .iter()
            .filter(|&&val| val | numbers[4] == *numbers[8])
            .next()
            .unwrap();
        numbers[6] = sixes
            .iter()
            .filter(|&&val| val | numbers[1] == *numbers[8])
            .next()
            .unwrap();
        numbers[9] = sixes
            .iter()
            .filter(|&&val| val | numbers[4] == *val)
            .next()
            .unwrap();
        numbers[0] = sixes
            .iter()
            .filter(|&&val| val != numbers[6] && val != numbers[9])
            .next()
            .unwrap();
        numbers[3] = fives
            .iter()
            .filter(|&&val| val | numbers[1] == *val)
            .next()
            .unwrap();
        numbers[5] = fives
            .iter()
            .filter(|&&val| val != numbers[2] && val != numbers[3])
            .next()
            .unwrap();

        let screen_iter = line_iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|str| str.chars().map(|char| quantify(char)).sum::<usize>());

        let mut result: usize = 0;
        for (i, val) in screen_iter.rev().enumerate() {
            let digit: usize = numbers.iter().position(|&&num| num == val).unwrap();
            result += digit * 10_usize.pow(i.try_into().unwrap());
        }
        // println!("{}", result);
        part2 += result;
    }
    println!("part 2 = {}", &part2);
}
