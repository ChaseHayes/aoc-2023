use std::fs;

pub fn get_answer() -> i32 {
    let contents = fs::read_to_string("./src/day1_puzzle1.txt")
        .expect("Should have been able to read the file");

    let contents_arr = contents.split("\n");

    let total = contents_arr
        .fold(0, |acc, x| acc + get_outer_digits(x));

    return total;
}

fn get_outer_digits(text: &str) -> i32 {
    let int_chars = text.chars();

    if int_chars.clone().count() == 0 {
        return 0;
    }

    let int_arr = int_chars
        .fold(Vec::new(), |mut acc, x: char| -> Vec<char> {
            let x_int = x.to_digit(10);
            match x_int {
                Some(_) => { acc.push(x); return acc },
                None => acc,
            }
        });

    return format!(
        "{}{}", 
        int_arr.first().unwrap(),
        int_arr.last().unwrap()
    )
        .parse::<i32>()
        .unwrap();
}

