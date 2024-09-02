use std::{collections::HashMap, fs};

pub fn get_answer() -> i32 {
    let contents = fs::read_to_string("./src/day1_puzzle1.txt")
        .expect("Should have been able to read the file");

    let contents_arr = contents.split("\n");

    let total = contents_arr
        .fold(0, |acc, x| acc + get_outer_digits(x));

    return total
}

fn get_outer_digits(text: &str) -> i32 {
    let number_positions = get_number_positions(text);
}

fn get_number_positions(text: &str) -> HashMap<int32, int32> {
    let search_terms = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
    ];
}
