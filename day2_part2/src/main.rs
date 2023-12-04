use std::fs;
use regex::Regex;

fn main() {
    let re_group = Regex::new(r"(\d+)\s+(?:(red)|(green)|(blue))").unwrap();
    let mut sum: u32 = 0;
    let contents: String = fs::read_to_string("src/input.txt").expect("could not read file");
    for game in contents.lines() {
        let mut red_max: u32 = 0;
        let mut green_max: u32 = 0;
        let mut blue_max: u32 = 0;
        for (_, [num, color])in re_group.captures_iter(game).map(|c| c.extract::<2>()) {
            let num = num.parse::<u32>().unwrap();
            match color {
                "red" => { if num > red_max { red_max = num; } },
                "green" => { if num > green_max { green_max = num; } },
                "blue" => { if num > blue_max { blue_max = num; } },
                _ => (),
            }
        }
        let power_of_min: u32 = red_max * green_max * blue_max;
        sum += power_of_min;
    }
    println!("the sum of all the powers is {}", sum);
}
