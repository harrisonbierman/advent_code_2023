use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("file not read");
    let mut sum: u32 = 0;
    for line in contents.lines() {
            let mut ch_total: String = String::new();
        for ch in line.chars() {
            match ch.to_digit(10) {
                Some(_) => ch_total.push(ch),
                None => (),
            }
        }
        if ch_total.len() == 1 {
            ch_total = ch_total.clone() + ch_total.as_str();
        }
        if ch_total.len() > 2 {
            let mut temp = ch_total.as_str().chars().nth(0).unwrap().to_string();
            temp.push(ch_total.as_str().chars().nth(ch_total.len() - 1).unwrap());
            ch_total = temp;
        }
        sum += ch_total.parse::<u32>().unwrap();
            //println!("{}", &ch_total)
    }
    println!("the total is {}", sum)
}
