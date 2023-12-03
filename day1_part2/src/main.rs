use std::fs;
use regex::Regex;

fn main() {
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_flip = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let contents = fs::read_to_string("src/input.txt").expect("file not read");
    let mut sum: u32 = 0;
    for line in contents.lines() {
        let first = word_to_digit(re.find(line).unwrap().as_str());
        let line_rev: String = line.chars().rev().collect();
        let last = word_to_digit(re_flip.find(line_rev.as_str()).unwrap().as_str());
        let num: u32 = format!("{}{}", first, last).parse().unwrap();
        sum += num;
    }
    println!("the calibration number is {sum}");
}

fn word_to_digit(word: &str) -> &str {
    match word {
        "one" | "eno" => "1",
        "two" | "owt" => "2",
        "three" | "eerht" => "3",
        "four"| "ruof" => "4",
        "five" | "evif" => "5",
        "six" | "xis" => "6",
        "seven" | "neves" => "7",
        "eight" | "thgie" => "8",
        "nine" | "enin" => "9",
        _ => word,
    }
}