use std::fs;
fn main() {
    let (k, l) = calculate_score();
    println!("Score: {} {}", k, l);
}

static ROCK: u8 = 1;
static PAPER: u8 = 2;
static SCISSORS: u8 = 3;
static LOSE: u8 = 0;
static WIN: u8 = 6;
static DRAW: u8 = 3;

fn calculate_score() -> (i32, i32) {
    let contents =
        fs::read_to_string("demo_input.txt").expect("Should have been able to read the file");
    let mut score = 0;
    let mut score2 = 0;
    for line in contents.split('\n') {
        score += match line.chars().next() {
            Some('A') => {
                if line.chars().nth(2) == Some('X') {
                    score2 += SCISSORS as i32 + LOSE as i32;
                    ROCK as i32 + DRAW as i32
                } else if line.chars().nth(2) == Some('Y') {
                    score2 += ROCK as i32 + DRAW as i32;
                    PAPER as i32 + WIN as i32
                } else {
                    score2 += PAPER as i32 + WIN as i32;
                    SCISSORS as i32 + LOSE as i32
                }
            }
            Some('B') => {
                if line.chars().nth(2) == Some('X') {
                    score2 += ROCK as i32 + LOSE as i32;
                    ROCK as i32 + LOSE as i32
                } else if line.chars().nth(2) == Some('Y') {
                    score2 += PAPER as i32 + DRAW as i32;
                    PAPER as i32 + DRAW as i32
                } else {
                    score2 += SCISSORS as i32 + WIN as i32;
                    SCISSORS as i32 + WIN as i32
                }
            }
            Some('C') => {
                if line.chars().nth(2) == Some('X') {
                    score2 += PAPER as i32 + LOSE as i32;
                    ROCK as i32 + WIN as i32
                } else if line.chars().nth(2) == Some('Y') {
                    score2 += SCISSORS as i32 + DRAW as i32;
                    PAPER as i32 + LOSE as i32
                } else {
                    score2 += ROCK as i32 + WIN as i32;
                    SCISSORS as i32 + DRAW as i32
                }
            }
            _ => 0,
        };
    }
    (score, score2)
}
