use std::fs;
fn main() {
    both_parts()
}

fn both_parts() {
    let contents: String =
        fs::read_to_string("demo_input.txt").expect("Should have been able to read the file");
    let mut contains: i32 = 0;
    let mut contains2: i32 = 0; // Part 2
    for line in contents.split('\n') {
        let range: Vec<&str> = line.split(',').collect();
        let m: Vec<&str> = range[0].split('-').collect();
        let n: Vec<&str> = range[1].split('-').collect();
        let mut numbers_contained_1: Vec<i32> = Vec::new();
        let mut numbers_contained_2: Vec<i32> = Vec::new();
        for i in m[0].parse::<i32>().unwrap()..m[1].parse::<i32>().unwrap() + 1 {
            numbers_contained_1.push(i);
        }
        for i in n[0].parse::<i32>().unwrap()..n[1].parse::<i32>().unwrap() + 1 {
            numbers_contained_2.push(i);
        }
        if numbers_contained_1
            .iter()
            .all(|item| numbers_contained_2.contains(item))
            || numbers_contained_2
                .iter()
                .all(|item| numbers_contained_1.contains(item))
        {
            contains += 1;
        }
        if numbers_contained_1 // Part 2
            .iter()
            .any(|x| numbers_contained_2.contains(x))
        {
            contains2 += 1;
        }
    }
    println!("{}, {}", contains, contains2);
}
