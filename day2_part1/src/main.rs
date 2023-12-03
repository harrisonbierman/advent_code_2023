use std::fs;
use regex::Regex;

const TARGET_RED: u32 = 12;
const TARGET_GREEN: u32 = 13;
const TARGET_BLUE: u32 = 14;

fn main() {
    let re_color = Regex::new(r"red|green|blue").unwrap();
    let re_num = Regex::new(r"\d+").unwrap();
    // the target game is 12 red cubes, 13 green cubes, 14 blue cubes
    let mut sum: usize = 0;
    let contents: String = fs::read_to_string("src/input.txt").expect("could not read file");
    for (game_id, game) in contents.lines().enumerate() {
        let mut is_possible = true;
        let game = game.to_string();
        let game = game.split(":").collect::<Vec<&str>>()[1];
        'subset: for subset in game.to_string().split(";") {
            for group in subset.split(",") {
                let num = re_num.find(group).unwrap().as_str().parse::<u32>().unwrap(); 
                //println!("{}", group);
                match re_color.find(group).unwrap().as_str(){
                    "red" => if num > TARGET_RED {
                        is_possible = false
                    },
                    "green" => if num > TARGET_GREEN {
                        is_possible = false
                    },
                    "blue" => if num > TARGET_BLUE {
                        is_possible = false
                    },
                    _ => println!("Color not found"),
                }
                if !is_possible {
                    break 'subset;
                }
            } 
        }
        if is_possible {
            //need to add one because input is index 1 and game_id is index 0
            sum += game_id + 1;
        }
    }
    println!("The sum of all possible games is {sum}");
}
