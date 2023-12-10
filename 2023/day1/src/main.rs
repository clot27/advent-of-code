use std::fs;

fn first_part() -> i32 {
    let contents: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut vec_nums: Vec<i32> = Vec::new();
    for i in contents {
        let k: Vec<_> = i.chars().filter_map(|c| c.to_digit(10)).collect();
        vec_nums.push(
            (k[0].to_string() + &k.last().expect("Something wrong").to_string())
                .parse::<i32>()
                .unwrap(),
        )
    }
    vec_nums.iter().sum()
}

fn second_part() -> i32 {
    let contents: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut vec_nums: Vec<i32> = Vec::new();
    for i in contents {
        let i = i
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        let k: Vec<_> = i.chars().filter_map(|c| c.to_digit(10)).collect();
        vec_nums.push(
            (k.first().to_string() + &k.last().expect("Something wrong").to_string())
                .parse::<i32>()
                .unwrap(),
        )
    }
    vec_nums.iter().sum()
}
fn main() {
    let sum1 = first_part();
    let sum2 = second_part();
    println!("Sum1 is {}", sum1);
    println!("Sum2 is {}", sum2);
}
