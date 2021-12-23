use std::{collections::HashMap, fs};

fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("could not read file");
    let raw_input = fs::read_to_string("input.txt").expect("could not read file");
    let mut lines_iter = raw_input.lines();
    let template: Vec<char> = lines_iter.next().unwrap_or("").chars().collect();
    let mut occurrences: HashMap<char, usize> = HashMap::new();

    for &ch in &template {
        let entry = occurrences.entry(ch).or_insert(0);
        (*entry) += 1;
    }

    lines_iter.next();
    let mut rules: HashMap<[char; 2], char> = HashMap::new();
    for line in lines_iter {
        let mut line_iter = line.split(" -> ");
        let tmp_key = line_iter.next().unwrap();
        let tmp_value = line_iter.next().unwrap();
        let container = tmp_value.chars().next().unwrap();
        let key = [
            tmp_key.chars().next().unwrap(),
            tmp_key.chars().nth(1).unwrap(),
        ];
        rules.entry(key).or_insert(container);
    }

    let mut pairs_counts: HashMap<[char; 2], usize> = HashMap::new();
    for ch in template.windows(2) {
        let entry = pairs_counts.entry([ch[0], ch[1]]).or_insert(0);
        (*entry) += 1;
    }

    for _ in 0..10 {
        let mut new_counts: HashMap<[char; 2], usize> = HashMap::new();
        for (&key, &val) in pairs_counts.iter() {
            let ch = rules[&key];
            let entry1 = new_counts.entry([key[0], ch]).or_insert(0);
            (*entry1) += val;
            let entry2 = new_counts.entry([ch, key[1]]).or_insert(0);
            (*entry2) += val;
            let occurrence = occurrences.entry(ch).or_insert(0);
            (*occurrence) += val;
        }
        pairs_counts = new_counts.clone();
    }

    let part1 = occurrences.values().max().unwrap() - occurrences.values().min().unwrap();
    println!("part 1 = {}", part1);

    for _ in 0..30 {
        let mut new_counts: HashMap<[char; 2], usize> = HashMap::new();
        for (&key, &val) in pairs_counts.iter() {
            let ch = rules[&key];
            let entry1 = new_counts.entry([key[0], ch]).or_insert(0);
            (*entry1) += val;
            let entry2 = new_counts.entry([ch, key[1]]).or_insert(0);
            (*entry2) += val;
            let occurrence = occurrences.entry(ch).or_insert(0);
            (*occurrence) += val;
        }
        pairs_counts = new_counts.clone();
    }

    let part2 = occurrences.values().max().unwrap() - occurrences.values().min().unwrap();
    println!("part 2 = {}", &part2);
}
