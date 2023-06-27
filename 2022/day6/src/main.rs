use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_one()
}

fn part_one() {
    let file: File = File::open("demo_input.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let vecc: Vec<char> = line.chars().collect::<Vec<char>>();
    part_two(&vecc); // Run part two here
    for (index, _i) in vecc.iter().enumerate() {
        let unique_vec: Vec<&char> = (index..index + 4).map(|i| &vecc[i]).collect::<Vec<_>>();
        if has_duplicate_characters(unique_vec) {
            println!("Part one required index is {}", index + 4);
            break;
        }
    }
}

fn part_two(vecc: &[char]) {
    for (index, _i) in vecc.iter().enumerate() {
        let unique_vec2: Vec<&char> = (index..index + 14).map(|i| &vecc[i]).collect::<Vec<_>>();
        if has_duplicate_characters(unique_vec2) {
            println!("Part two required index is {}", index + 14);
            break;
        }
    }
}

fn has_duplicate_characters(vector: Vec<&char>) -> bool {
    let mut seen: HashSet<&char> = HashSet::new();

    for i in vector {
        if seen.contains(i) {
            return false;
        }
        seen.insert(i);
    }
    true
}
