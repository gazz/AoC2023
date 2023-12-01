use std::env;
use std::fs;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let calibration = calibrate(&contents);
    println!("Result {calibration}");
}

fn calibrate(input : &str) -> u64 {

    let re = Regex::new(r"\d").unwrap();

    // split by lines
    let numbers : Vec<_> = input.lines().map(|line| { 
        let matches: Vec<_> = re.find_iter(line.trim()).map(|m| m.as_str()).collect();

        let first = matches.first().unwrap();
        let last = matches.last().unwrap();
        let formatted = format!("{}{}", first, last); 
        let out = u64::from_str(&formatted).unwrap();
        // let last = &matches.last();
        println!("Line: {}, First digit: {}, last digit: {}, formatted: {}, out: {}",
            line, first, last, formatted, out);
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
        treb7uchet
        testwithout";

    assert_eq!(calibrate(input), 142);
}