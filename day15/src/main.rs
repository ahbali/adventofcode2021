use std::{collections::VecDeque, fs};

fn neighbors((i, j): (isize, isize)) -> [(isize, isize); 4] {
    [(i, j + 1), (i, j - 1), (i + 1, j), (i - 1, j)]
}

fn risk_of((i, j): (usize, usize), board: &Vec<Vec<u8>>) -> u8 {
    let rows = board.len();
    let cols = board[0].len();
    let (reminder_i, reminder_j) = (i % cols, j % rows);
    let (div_i, div_j) = (i / cols, j / rows);
    (1 + (((board[reminder_j][reminder_i] - 1) as usize + (div_i + div_j)) % 9)) as u8
}

fn fill_weights(
    (i0, j0): (isize, isize),
    rows: usize,
    cols: usize,
    board: &Vec<Vec<u8>>,
    weights: &mut Vec<Vec<(usize, usize, usize)>>,
    visited: Vec<(usize, usize)>,
) {
    if visited.contains(&(i0 as usize, j0 as usize)) {
        return;
    }
    let mut new_visited: Vec<(usize, usize)> = vec![];
    new_visited.extend(visited);
    new_visited.push((i0 as usize, j0 as usize));
    for &(i, j) in neighbors((i0, j0))
        .iter()
        .filter(|&&(i, j)| i >= 0 && j >= 0 && i < cols as isize && j < rows as isize)
    {
        let new_weight =
            risk_of((i as usize, j as usize), board) as usize + weights[j0 as usize][i0 as usize].0;
        if new_weight < weights[j as usize][i as usize].0 {
            weights[j as usize][i as usize] = (new_weight, i0 as usize, j0 as usize);
            new_visited.push((i as usize, j as usize));
            fill_weights((i, j), rows, cols, board, weights, new_visited.clone());
        }
    }
}

fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("could not read file");
    let raw_input = fs::read_to_string("input.txt").expect("could not read file");
    let board: Vec<Vec<u8>> = raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();
    let rows_p1 = board.len();
    let cols_p1 = board[0].len();
    let mut weights_p1: Vec<Vec<(usize, usize, usize)>> =
        vec![vec![(usize::MAX, 0, 0); cols_p1]; rows_p1];
    weights_p1[0][0] = (board[0][0] as usize, 0, 0);
    for i in 0..cols_p1 as isize {
        for j in 0..rows_p1 as isize {
            fill_weights((i, j), rows_p1, cols_p1, &board, &mut weights_p1, vec![]);
        }
    }
    let (mut i, mut j) = (cols_p1 - 1, rows_p1 - 1);
    let mut risk = 0;
    while (i, j) != (0, 0) {
        risk += board[j][i] as usize;
        let (x, y) = (weights_p1[j][i].1, weights_p1[j][i].2);
        i = x;
        j = y;
    }
    println!("part 1 = {}", risk);

    let rows_p2 = board.len() * 5;
    let cols_p2 = board[0].len() * 5;
    let mut weights_p2: Vec<Vec<(usize, usize, usize)>> =
        vec![vec![(usize::MAX, 0, 0); cols_p2]; rows_p2];
    weights_p2[0][0] = (0, 0, 0);
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((0, 0));
    while !queue.is_empty() {
        let (i0, j0) = queue.pop_front().unwrap();
        for (i, j) in neighbors((i0 as isize, j0 as isize))
            .iter()
            .filter(|&&(i, j)| i >= 0 && j >= 0 && i < cols_p2 as isize && j < rows_p2 as isize)
            .map(|&(x, y)| (x as usize, y as usize))
        {
            let new_weight = risk_of((i, j), &board) as usize + weights_p2[j0][i0].0;
            if new_weight < weights_p2[j][i].0 {
                weights_p2[j][i] = (new_weight, i0, j0);
                queue.push_back((i, j));
            }
        }
    }
    let part2 = weights_p2[rows_p2 - 1][cols_p2 - 1].0;
    println!("part dij 2 = {}", &part2);
}
