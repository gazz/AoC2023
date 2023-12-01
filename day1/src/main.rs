use std::env;
use std::fs;
use fancy_regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let calibration = calibrate(&contents);
    println!("Result {calibration}");
}


fn as_digit(digit_or_str: &str) -> u64 {
    let number_mapping = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    match number_mapping.get(digit_or_str) {
        Some(digit) => *digit,
        None => u64::from_str(digit_or_str).unwrap()
    }
}

fn calibrate(input : &str) -> u64 {

    let re = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();

    // split by lines
    let numbers : Vec<_> = input.lines().map(|line| { 
        let matches: Vec<_> = re.captures_iter(line.trim())
            .map(|m| m.unwrap().get(1).unwrap().as_str()).collect();

        println!("Matches: {:?}", matches);

        let first = as_digit(matches.first().unwrap());
        let last = as_digit(matches.last().unwrap());
        let formatted = format!("{}{}", first, last); 
        println!("Line: {}, First digit: {}, last digit: {}, formatted: {}",
            line, first, last, formatted);
        let out = u64::from_str(&formatted).unwrap();
        println!("out: {}", out);
        out
    }).collect();

    numbers.into_iter().sum::<u64>()
}

#[test]
fn test_calibrate() {
    let input = 
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    assert_eq!(calibrate(input), 142);
}

#[test]
fn test_calibrate_words() {
    let input = 
        "two1nine
        eightwothree
        eighthree
        sevenine
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
    assert_eq!(calibrate(input), 281 + 83 + 79);
}

// #[test]
// fn test_fancy_re() {

//     let re = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
//     let matches : Vec<_> = re.captures_iter("eighthree")
//         .map(|m| m.unwrap())
//         .collect();

//     println!("Matches: {:?}", matches);
//     let strMatches : Vec<_> = matches.iter()
//         .map(|m| m.get(1).unwrap().as_str())
//         .collect();
//     println!("Str Matches: {:?}", strMatches);

//     assert_eq!(0, 1);
// }