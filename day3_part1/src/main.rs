use regex::Regex;
use std::fs;

static EMPTY_VEC: Vec<char> = Vec::new();

fn main() {
    let contents = fs::read_to_string("src/input.txt").unwrap();
    let lines = contents.lines();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for (row, line) in lines.clone().enumerate() {
        grid.push(Vec::new());
        grid[row].push('.'); // left grid padding
        for ch in line.chars() {
                grid[row].push(ch)
        }
    }

    grid.insert(0, Vec::new()); // upper grid padding

    let mut sum: u32 = 0;
    let re_num = Regex::new(r"(\d+)").unwrap();
    let re_sym = Regex::new(r"-|\*|/|#|&|\+|@|=|\$|%").unwrap();

    for (row, line) in lines.clone().enumerate() {
        for num in re_num.find_iter(line) {
            let num_col_start: usize = num.start() + 1; // account for padding
            let num_col_end: usize = num.end(); // .end() offsets by one
            let is_num_start_part: bool = is_symbol_surrounding(&(row + 1), num_col_start, &grid, &re_sym); // rows account for padding
            let is_num_end_part: bool = is_symbol_surrounding(&(row + 1), num_col_end, &grid, &re_sym);
            if is_num_start_part || is_num_end_part {
                sum += num.as_str().parse::<u32>().unwrap();
            }
        }
    }
    println!("The sum of all the parts is {}", sum);
}

fn is_symbol_surrounding(row: &usize, col: usize, grid: &Vec<Vec<char>>, re_sym: &Regex) -> bool {
    let mut surrounding_symbols: String = String::new();
    surrounding_symbols.push(*grid.get(row - 1).unwrap_or(&EMPTY_VEC).get(col    ).unwrap_or(&'.')); // north
    surrounding_symbols.push(*grid.get(row - 1).unwrap_or(&EMPTY_VEC).get(col + 1).unwrap_or(&'.')); // north east
    surrounding_symbols.push(*grid.get(*row   ).unwrap_or(&EMPTY_VEC).get(col + 1).unwrap_or(&'.')); // east
    surrounding_symbols.push(*grid.get(row + 1).unwrap_or(&EMPTY_VEC).get(col + 1).unwrap_or(&'.')); // south east
    surrounding_symbols.push(*grid.get(row + 1).unwrap_or(&EMPTY_VEC).get(col    ).unwrap_or(&'.')); // south 
    surrounding_symbols.push(*grid.get(row + 1).unwrap_or(&EMPTY_VEC).get(col - 1).unwrap_or(&'.')); // south west
    surrounding_symbols.push(*grid.get(*row   ).unwrap_or(&EMPTY_VEC).get(col - 1).unwrap_or(&'.')); // west
    surrounding_symbols.push(*grid.get(row - 1).unwrap_or(&EMPTY_VEC).get(col - 1).unwrap_or(&'.')); // north west
    re_sym.is_match(&surrounding_symbols)
}

// preliminary function that is not used in final calulation
// unique symbols found were  '-', '*', '/', '#', '&', '+', '@', '=', '$', '%'
fn find_unique_symbols(string: String) {
    let mut unique_symbols: Vec<char> = Vec::new();
    for ch in string.chars() {
        if !unique_symbols.contains(&ch) {
           unique_symbols.push(ch); 
        } 
    }
    println!("these are all the unique symbols in the string {:?}", unique_symbols);
}