use std::error::Error;
use std::fs;
use regex::{ Regex, Match };
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {

    // creates hashmap where
    // key: card number, 
    // value: number of winning matches
    let contents: String = fs::read_to_string("day4_part2/src/input.txt")?;
    let re_num: Regex = Regex::new(r"\d+")?;
    let mut match_map: HashMap<usize, usize> = HashMap::new();
    for (card_num, line) in contents.lines().enumerate() {
        let card_data: Vec<&str> = line.split([':','|']).collect();

        //creates an array for the winner numbers
        // pasrsed from the second section of card_data
        let mut win_arr: Vec<usize> = Vec::new();
        for win_num in re_num.find_iter(card_data[1]).map(|c: Match<'_>| c.as_str()) {
            win_arr.push(win_num.parse::<usize>()?)
        }

        // creates array for numbers that will be matched
        // with winning numbers.
        //parsed from third section of card_data
        let mut match_arr: Vec<usize> = Vec::new();
        for match_num in re_num.find_iter(card_data[2]).map(|c: Match<'_>| c.as_str()) {
            match_arr.push(match_num.parse::<usize>()?)
        }
        
        // finds sum of all winning matches for card
        let mut match_count_sum:usize = 0;
        for win_num in win_arr {
            for match_num in match_arr.to_owned() {
                if win_num == match_num {
                   match_count_sum += 1_usize; 
                }
            }
        }
        match_map.insert( card_num + 1, match_count_sum); // adjusted for index 1 
    }
    
    let mut sum: usize = 0;
    for card in &match_map {
        let matches: usize = calculate_matches(*card.0, &match_map);
        sum += matches;
    }

    println!("The sum of all the matches is {}", sum);

    Ok(())
}

// recursivly called function that sums the scratchcard
// paths from a single scratch card input
fn calculate_matches(card_num: usize, match_map: &HashMap<usize, usize>) -> usize {
    let mut sum: usize = 1;
    let match_count: usize = *match_map.get(&card_num).unwrap_or(&0_usize);
    for i in 1_usize..=match_count {
        let next_card = card_num + i;
        let branch_sum = calculate_matches(next_card, match_map);
        sum += branch_sum;
    }

    sum
}