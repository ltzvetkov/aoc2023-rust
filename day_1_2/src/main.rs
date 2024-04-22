use std::char;
use std::fs;

fn main() {
    let input_file = "input";

    let mut sum = 0;

    for line in fs::read_to_string(input_file).unwrap().lines() {
        let mut v: Vec<char> = Vec::new();

        let line_chars: Vec<char> = line.chars().collect();

        let mut i: usize = 0;

        while i < line_chars.len() {
            let c = line_chars.get(i).unwrap().to_owned();

            if c.is_digit(10) {
                v.push(c);
            } else if let Some(c) = is_string_digit(&line_chars, i) {
                v.push(c);
            }

            i += 1;
        }

        println!("Numbers: {:?}", v);

        let mut string = String::from("");
        string.push(v.first().unwrap().to_owned());
        string.push(v.last().unwrap().to_owned());

        let number = string.parse::<i32>().unwrap();
        sum = sum + number;
    }

    println!("Sum: {sum}");
}

fn is_string_digit(line_chars: &Vec<char>, i: usize) -> Option<char> {
    let mut string = String::from("");
    string.push(line_chars.get(i)?.to_owned());
    string.push(line_chars.get(i + 1)?.to_owned());
    string.push(line_chars.get(i + 2)?.to_owned());

    match string.as_str() {
        "one" => Some('1'),
        "two" => Some('2'),
        "six" => Some('6'),
        _ => {
            string.push(line_chars.get(i + 3)?.to_owned());

            match string.as_str() {
                "four" => Some('4'),
                "five" => Some('5'),
                "nine" => Some('9'),
                _ => {
                    string.push(line_chars.get(i + 4)?.to_owned());

                    match string.as_str() {
                        "three" => Some('3'),
                        "seven" => Some('7'),
                        "eight" => Some('8'),
                        _ => None,
                    }
                }
            }
        }
    }
}
