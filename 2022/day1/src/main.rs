use std::fs;

fn main() {
    let mut k: Vec<i32> = extract_input_to_vec();
    let max_value: &Option<&i32> = &k.iter().max();
    match max_value {
        // For first part of puzzle
        Some(max) => println!("Max value: {}", max),
        None => println!("Vector is empty"),
    }
    k.sort_by(|a: &i32, b: &i32| b.cmp(a)); // For second part of puzzle
    let top_three: i32 = k.iter().take(3).sum();
    println!("Top three: {}", top_three);
}

fn extract_input_to_vec() -> Vec<i32> {
    let contents: String =
        fs::read_to_string("demo_input.txt").expect("Should have been able to read the file");
    let mut elf: Vec<i32> = Vec::new();
    let mut initial: i32 = 0;
    for line in contents.split('\n') {
        if !line.is_empty() {
            initial += line.parse::<i32>().unwrap();
        } else {
            elf.push(initial);
            initial = 0;
            continue;
        }
    }
    elf
}
