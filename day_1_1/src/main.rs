use std::char;
use std::fs;

fn main() {
    let input_file = "input";

    let mut sum = 0;

    for line in fs::read_to_string(input_file).unwrap().lines() {
        let mut v: Vec<char> = Vec::new();

        for c in line.chars() {
            if c.is_digit(10) {
                v.push(c);
            }
        }

        let mut string = String::from("");
        string.push(v.first().unwrap().to_owned());
        string.push(v.last().unwrap().to_owned());

        let number = string.parse::<i32>().unwrap();
        sum = sum + number;
    }

    println!("Sum: {sum}");
}
