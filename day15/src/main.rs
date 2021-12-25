use std::{fmt::Debug, fs};

fn neighbors((i, j): (isize, isize)) -> [(isize, isize); 4] {
    [(i, j + 1), (i, j - 1), (i + 1, j), (i - 1, j)]
}

fn print_board<T: Debug>(board: &Vec<Vec<T>>) {
    for line in board.iter() {
        println!("{:?}", line);
    }
}

fn fill_weights(
    (i0, j0): (isize, isize),
    lines: usize,
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
        .filter(|&&(i, j)| i >= 0 && j >= 0 && i < cols as isize && j < lines as isize)
    {
        let new_weight =
            board[j as usize][i as usize] as usize + weights[j0 as usize][i0 as usize].0;
        if new_weight < weights[j as usize][i as usize].0 {
            weights[j as usize][i as usize] = (new_weight, i0 as usize, j0 as usize);
            new_visited.push((i as usize, j as usize));
            fill_weights((i, j), lines, cols, board, weights, new_visited.clone());
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
    let rows = board.len();
    let cols = board[0].len();
    let mut weights: Vec<Vec<(usize, usize, usize)>> = vec![vec![(usize::MAX, 0, 0); cols]; rows];
    weights[0][0] = (board[0][0] as usize, 0, 0);
    for i in 0..cols as isize {
        for j in 0..rows as isize {
            fill_weights((i, j), rows, cols, &board, &mut weights, vec![]);
        }
    }
    let (mut i, mut j) = (cols - 1, rows - 1);
    let mut risk = 0;
    while (i, j) != (0, 0) {
        risk += board[j][i] as usize;
        let (x, y) = (weights[j][i].1, weights[j][i].2);
        i = x;
        j = y;
    }

    // print_board(&weights);
    // for line in weights.iter() {
    //     for &(weight, i, j) in line {
    //         if weight == usize::MAX {
    //             print!("[***,({},{})] ", i, &j);
    //         } else {
    //             print!("[{},({},{})] ", weight, &i, &j);
    //         }
    //     }
    //     println!();
    // }
    // println!("board[0] = {}", board[0][0]);
    // println!("board[][] = {}", board[rows - 1][cols - 1]);
    // println!("weight[][] = {:?}", weights[rows - 1][cols - 1]);
    println!("part 1 = {}", risk);
}
