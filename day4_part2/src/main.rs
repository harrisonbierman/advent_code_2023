use std::rc::{ Rc, Weak };
use std::error::Error;
use std::fs;
use regex::{ Regex, Match };


pub struct Card {
    pub parent: Weak<Card>,
    pub number: u32,
    pub children: Vec<Option<Rc<Card>>>
}

impl Card {
    pub fn new(number: u32,) -> Card{
        Card {
            parent: Weak::new(),
            number,
            children: Vec::new(), 
        }
    }

    pub fn add_parent(&mut self, parent_card: &Rc<Card>) {
        self.parent = Rc::<Card>::downgrade(parent_card);
    }
}

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
                    card_matches.push(match_num)
                }
            }
        }


    }

    println!("The sum of points is {sum}");

    Ok(())
}
