use advent_of_code_2022_05::find_top_crates_6000;
use advent_of_code_2022_05::find_top_crates_6001;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut crates = find_top_crates_6000(&contents);
    println!("The top crates using CrateMover 6000 are: {crates}");

    crates = find_top_crates_6001(&contents);
    println!("The top crates using CrateMover 6001 are: {crates}");
}
