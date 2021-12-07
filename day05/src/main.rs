use std::{collections::HashMap, fs};

fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("error opening file");
    let raw_input = fs::read_to_string("input.txt").expect("error opening file");
    let input = raw_input.lines().map(|line| {
        line.split(" -> ")
            .flat_map(|str| str.split(','))
            .filter_map(|el| el.parse::<usize>().ok())
    });

    //
    // part 1
    //
    let mut world_map1: HashMap<(usize, usize), usize> = HashMap::new();
    for mut line in input.clone() {
        let x1 = line.next().unwrap();
        let y1 = line.next().unwrap();
        let x2 = line.next().unwrap();
        let y2 = line.next().unwrap();
        if x1 == x2 {
            for i in y1.min(y2)..=y1.max(y2) {
                let entry = world_map1.entry((x1, i)).or_insert(0);
                *entry += 1;
            }
        } else if y1 == y2 {
            for i in x1.min(x2)..=x1.max(x2) {
                let entry = world_map1.entry((i, y1)).or_insert(0);
                *entry += 1;
            }
        }
    }
    println!(
        "part 1= {}",
        world_map1.values().filter(|&&val| val > 1).count()
    );

    //
    // part 2
    //
    let mut world_map2: HashMap<(usize, usize), usize> = HashMap::new();
    for mut line in input {
        let x1 = line.next().unwrap();
        let y1 = line.next().unwrap();
        let x2 = line.next().unwrap();
        let y2 = line.next().unwrap();
        if x1 == x2 {
            for i in y1.min(y2)..=y1.max(y2) {
                let entry = world_map2.entry((x1, i)).or_insert(0);
                *entry += 1;
            }
        } else if y1 == y2 {
            for i in x1.min(x2)..=x1.max(x2) {
                let entry = world_map2.entry((i, y1)).or_insert(0);
                *entry += 1;
            }
        } else {
            for i in 0..=(x1 as isize - x2 as isize).abs() as usize {
                if x1 < x2 {
                    if y1 < y2 {
                        let entry = world_map2.entry((x1 + i, y1 + i)).or_insert(0);
                        *entry += 1;
                    } else {
                        let entry = world_map2.entry((x1 + i, y1 - i)).or_insert(0);
                        *entry += 1;
                    }
                }
                if x1 > x2 {
                    if y1 > y2 {
                        let entry = world_map2.entry((x1 - i, y1 - i)).or_insert(0);
                        *entry += 1;
                    } else {
                        let entry = world_map2.entry((x1 - i, y1 + i)).or_insert(0);
                        *entry += 1;
                    }
                }
            }
        }
    }
    println!(
        "part 2= {}",
        world_map2.values().filter(|&&val| val > 1).count()
    );
}
