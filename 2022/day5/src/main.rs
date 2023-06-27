use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_one();
}

fn part_one() {
    let file = File::open("demo_input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut stack_no = 0;
    let mut stack_lines = false;
    let mut stacks: Vec<String> = Vec::new();
    for (_line_no, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                let vec_of_lines = line.chars().collect::<Vec<char>>();
                if !stack_lines {}
                if !vec_of_lines.is_empty() && vec_of_lines[1] == '1' {
                    stack_no += line.chars().filter(|&c| c.is_numeric()).count();
                    stacks = vec![String::from(""); stack_no];
                    stack_lines = true;
                }
            }
            Err(_) => println!("Error occurred"),
        }
    }
    println!("{:?}", stacks);
}
