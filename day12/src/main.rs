use std::{collections::HashMap, fs, iter};

fn find_paths_part1(graph: &HashMap<&str, Vec<&str>>, node: &str, visited: Vec<&str>) -> usize {
    if node == "end" {
        // println!("{:?}", visited);
        return 1;
    }
    let mut paths = 0_usize;
    let mut children: Vec<&str> = graph[node]
        .iter()
        .filter(|val| !visited.contains(val))
        .cloned()
        .collect();
    if children.len() == 0 {
        return 0;
    }
    while children.len() > 0 {
        let child = children.pop().unwrap();
        if child.chars().all(char::is_lowercase) {
            let new_visited: Vec<&str> = visited.iter().cloned().chain(iter::once(child)).collect();
            paths += find_paths_part1(graph, child, new_visited);
        } else {
            paths += find_paths_part1(graph, child, visited.clone());
        }
    }

    return paths;
}

fn find_paths_part2(
    graph: &HashMap<&str, Vec<&str>>,
    node: &str,
    visited: Vec<&str>,
    double: bool,
) -> usize {
    if node == "end" {
        // println!("{:?}", visited);
        return 1;
    }
    let mut paths = 0_usize;
    let mut children: Vec<&str> = if double {
        graph[node]
            .iter()
            .filter(|val| !visited.contains(val))
            .cloned()
            .collect()
    } else {
        graph[node]
            .iter()
            .filter(|&&val| val != "start")
            .cloned()
            .collect()
    };
    if children.len() == 0 {
        return 0;
    }
    while children.len() > 0 {
        let child = children.pop().unwrap();
        if child.chars().all(char::is_lowercase) {
            if double {
                let new_visited: Vec<&str> =
                    visited.iter().cloned().chain(iter::once(child)).collect();
                paths += find_paths_part2(graph, child, new_visited, double);
            } else if visited.contains(&child) {
                paths += find_paths_part2(graph, child, visited.clone(), true);
            } else {
                let new_visited: Vec<&str> =
                    visited.iter().cloned().chain(iter::once(child)).collect();
                paths += find_paths_part2(graph, child, new_visited, double);
            }
        } else {
            paths += find_paths_part2(graph, child, visited.clone(), double);
        }
    }

    return paths;
}

fn main() {
    // let raw_input = fs::read_to_string("sample1.txt").expect("could not read file");
    // let raw_input = fs::read_to_string("sample2.txt").expect("could not read file");
    // let raw_input = fs::read_to_string("sample3.txt").expect("could not read file");
    let raw_input = fs::read_to_string("input.txt").expect("could not read file");

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in raw_input.lines() {
        let mut line_iter = line.split('-');
        let key = line_iter.next().unwrap();
        let value = line_iter.next().unwrap();
        let entry1 = graph.entry(key).or_default();
        (*entry1).push(value);
        let entry2 = graph.entry(value).or_default();
        (*entry2).push(key);
    }

    let part1 = find_paths_part1(&graph, "start", vec!["start"]);
    println!("part 1 = {}", part1);

    let part2 = find_paths_part2(&graph, "start", vec!["start"], false);
    println!("part 2 = {}", &part2);
}
