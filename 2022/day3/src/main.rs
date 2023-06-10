use std::fs;
fn main() {
    let score = part_one();
    let score2 = part_two();
    println!("{}, {}", score, score2);
}

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn read_file_to_string() -> String {
    fs::read_to_string("demo_input.txt").expect("Should have been able to read the file")
}

fn part_one() -> i32 {
    let contents = read_file_to_string();
    let mut score: i32 = 0;
    for line in contents.split('\n') {
        let common = find_common_in_two(line.split_at(line.len() / 2));
        for i in common {
            score += ASCII_LOWER
                .iter()
                .position(|&k| k == i.to_ascii_lowercase())
                .unwrap() as i32;
            if i.is_lowercase() {
                score += 1;
            } else {
                score += 27;
            }
        }
    }
    score
}

fn part_two() -> i32 {
    let contents = read_file_to_string();
    let contents: Vec<&str> = contents.split('\n').collect();
    let mut to_append_vec: Vec<&str> = Vec::new();
    let mut score: i32 = 0;
    for line in contents {
        if to_append_vec.len() < 3 {
            to_append_vec.push(line);
            continue;
        }
        let common = find_common_in_three((to_append_vec[0], to_append_vec[1], to_append_vec[2]));
        to_append_vec.clear();
        to_append_vec.push(line);
        score += ASCII_LOWER
            .iter()
            .position(|&k| k == common[0].to_ascii_lowercase())
            .unwrap() as i32;
        if common[0].is_lowercase() {
            score += 1;
        } else {
            score += 27;
        }
    }

    score
}

fn find_common_in_two((str1, str2): (&str, &str)) -> Vec<char> {
    let mut vec1: Vec<char> = str1.chars().collect();
    let mut vec2: Vec<char> = str2.chars().collect();
    vec1.sort_unstable();
    vec2.sort_unstable();
    vec1.dedup();
    vec2.dedup();
    vec1.into_iter()
        .filter(|&i| vec2.contains(&i))
        .collect::<Vec<char>>()
}

fn find_common_in_three((str1, str2, str3): (&str, &str, &str)) -> Vec<char> {
    let mut vec1: Vec<char> = str1.chars().collect();
    let mut vec2: Vec<char> = str2.chars().collect();
    let mut vec3: Vec<char> = str3.chars().collect();
    vec1.sort_unstable();
    vec2.sort_unstable();
    vec3.sort_unstable();
    vec1.dedup();
    vec2.dedup();
    vec3.dedup();
    vec1.into_iter()
        .filter(|&i| vec2.contains(&i) && vec3.contains(&i))
        .collect::<Vec<char>>()
}
