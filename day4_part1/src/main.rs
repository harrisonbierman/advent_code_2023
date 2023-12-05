use std::error::Error;
use std::fs;
use regex::{ Regex, Match };

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string("src/input.txt")?;
    let re_num: Regex = Regex::new(r"\d+")?;
    let mut sum: u32 = 0;
    for line in contents.lines() {
        let three_parts: Vec<&str> = line.split([':','|']).collect();
        let mut win_arr: Vec<u32> = Vec::new();
        for win_num in re_num.find_iter(three_parts[1]).map(|c: Match<'_>| c.as_str()) {
            win_arr.push(win_num.parse::<u32>()?)
        }

        let mut match_arr: Vec<u32> = Vec::new();
        for match_num in re_num.find_iter(three_parts[2]).map(|c: Match<'_>| c.as_str()) {
            match_arr.push(match_num.parse::<u32>()?)
        }

        let mut win_count: u32 = 0;
        for win_num in &win_arr {
            for match_num in &match_arr {
                if win_num == match_num {
                    win_count += 1;
                }
            }
        }

        let mut points: u32 = 0;
        if win_count != 0 { 
            points = 2_u32.pow(win_count - 1);
        }

        sum += points;
    }

    println!("The sum of points is {sum}");

    Ok(())
}