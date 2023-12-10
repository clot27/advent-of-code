use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn first_part() -> (i32, i32) {
    let contents: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let color_pattern = Regex::new(r"([0-9]{1,2}) (blue|red|green)").unwrap();
    let mut possible_games: Vec<i32> = vec![];
    let mut power: Vec<i32> = vec![];
    'outer: for i in contents {
        let each_set = i.split(";").collect::<Vec<&str>>();
        let mut red: Vec<i32> = vec![];
        let mut blue: Vec<i32> = vec![];
        let mut green: Vec<i32> = vec![];
        for k in each_set {
            let mut counts: HashMap<String, i32> = HashMap::new();
            for caps in color_pattern.captures_iter(k) {
                let count = caps[1].parse::<i32>().unwrap();
                let color = caps[2].to_string();
                match color.as_str() {
                    "red" => red.push(if count == 0 { 1 } else { count }),
                    "blue" => blue.push(if count == 0 { 1 } else { count }),
                    "green" => green.push(if count == 0 { 1 } else { count }),
                    _ => println!("Something wrong"),
                }
                counts.insert(color, count);
            }
            // Comment out this block to get correct answer for part 2
            if counts.get("blue").unwrap_or(&0) > &14
                || counts.get("green").unwrap_or(&0) > &13
                || counts.get("red").unwrap_or(&0) > &12
            {
                continue 'outer;
            }
        }
        power.push(
            red.iter().max().unwrap() * blue.iter().max().unwrap() * green.iter().max().unwrap(),
        );
        let game_no_pattern = Regex::new(r"[0-9]{1,3}:").unwrap();
        let game_no = game_no_pattern
            .find(&i)
            .unwrap()
            .as_str()
            .replace(":", "")
            .parse::<i32>()
            .unwrap();
        possible_games.push(game_no);
    }
    (possible_games.iter().sum(), power.iter().sum())
}

fn main() {
    let sum1 = first_part();
    println!("{:?}", sum1);
}
