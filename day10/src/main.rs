use std::fs;

const OPENING_CHARS: &str = "([{<";
const CLOSING_CHARS: &str = ")]}>";

fn get_error_score(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn get_completion_score(ch: char) -> usize {
    match ch {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

fn closing_char(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn main() {
    // let raw_input = fs::read_to_string("sample.txt").expect("could not read file");
    let raw_input = fs::read_to_string("input.txt").expect("could not read file");
    let mut err_score: usize = 0;
    let mut completion_scores: Vec<usize> = vec![];

    for line in raw_input.lines() {
        let mut stack: Vec<char> = vec![];
        for char in line.chars() {
            if OPENING_CHARS.contains(char) {
                stack.push(char);
            } else if CLOSING_CHARS.contains(char) {
                if let Some(&ch) = stack.last() {
                    if char == closing_char(ch) {
                        stack.pop();
                    } else {
                        err_score += get_error_score(char);
                        stack.clear();
                        break;
                    }
                } else {
                    err_score += get_error_score(char);
                    break;
                }
            }
        }
        if stack.len() > 0 {
            let score = stack
                .iter()
                .rev()
                .fold(0usize, |acc, &ch| (acc * 5) + get_completion_score(ch));
            completion_scores.push(score);
        }
    }

    println!("part 1 = {}", err_score);

    completion_scores.sort();
    println!(
        "part 2 = {}",
        completion_scores[completion_scores.len() / 2]
    );
}
