use std::fs;

fn main() {
    // the target game is 12 red cubes, 13 green cubes, 14 blue cubes
    let sum: u32 = 0;
    let contents = fs::read_to_string("src/bin/day_2/test.txt").expect("could not read file");
    for line in contents.lines() {
        println!("{}", line);
    } 
}