use std::fs;

const ROW_SIZE: usize = 5;
const COLUMN_SIZE: usize = 5;
const BOARD_SIZE: usize = ROW_SIZE * COLUMN_SIZE;

#[derive(Debug)]
struct Board {
    board: [isize; BOARD_SIZE],
}

impl Board {
    fn new_from_vec(vector: Vec<isize>) -> Self {
        assert!(vector.len() == BOARD_SIZE, "problem parsing board");
        let board: [isize; BOARD_SIZE] = match vector.try_into() {
            Ok(v) => v,
            Err(e) => panic!("{:?}", e),
        };
        Board { board }
    }

    fn check_rows(&self) -> Option<isize> {
        // check if there is a row filled with -1s
        let mut sum = 0isize;
        for row in 0..ROW_SIZE {
            for col in 0..COLUMN_SIZE {
                sum += self.board[ROW_SIZE * row + col];
            }
            if sum == -5 {
                return Some(self.unmarked_sum());
            }
            sum = 0;
        }
        None
    }
    fn check_columns(&self) -> Option<isize> {
        // check if there is a column filled with -1s
        let mut sum = 0isize;
        for col in 0..COLUMN_SIZE {
            for row in 0..ROW_SIZE {
                sum += self.board[ROW_SIZE * row + col];
            }
            if sum == -5 {
                return Some(self.unmarked_sum());
            }
            sum = 0;
        }
        None
    }

    fn unmarked_sum(&self) -> isize {
        // return sum of elements of the board ignoring -1s
        self.board.iter().filter(|&&x| x != -1).sum::<isize>()
    }

    fn mark(&mut self, number: &isize) -> Option<isize> {
        // if number is found on the board it will be changed to -1
        if let Some(pos) = self.board.iter().position(|x| x == number) {
            self.board[pos] = -1;
        }
        if let Some(num) = self.check_columns() {
            return Some(num);
        }
        if let Some(num) = self.check_rows() {
            return Some(num);
        }
        None
    }
}

fn main() {
    let raw_input = fs::read_to_string("input.txt").expect("could not open file");
    // let raw_input = fs::read_to_string("sample.txt").expect("could not open file");
    let mut first_iter = raw_input.split("\n\n");
    let random_numbers: Vec<isize> = first_iter
        .next()
        .unwrap()
        .split(',')
        .filter_map(|str| str.parse().ok())
        .collect();
    let mut boards: Vec<Board> = first_iter
        .map(|str| {
            str.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<isize>>()
        })
        .map(|vec| Board::new_from_vec(vec))
        .collect();

    let mut finished: Vec<usize> = vec![];
    let mut data: Vec<(usize, isize, isize)> = vec![];
    for &number in random_numbers.iter() {
        for (index, board) in boards.iter_mut().enumerate() {
            if !finished.contains(&index) {
                if let Some(unmarked_sum) = board.mark(&number) {
                    finished.push(index);
                    data.push((index, unmarked_sum, number));
                }
            }
        }
    }

    println!("part 1= {}", data[0].1 * data[0].2);
    let last_element = data.pop().unwrap();
    print!("part 2= {}", last_element.1 * last_element.2);
}
