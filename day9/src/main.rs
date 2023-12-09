use std::str::FromStr;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str) {
    let histories : Vec<Vec<isize>> = contents.lines()
        .map(|l| 
            l.split(" ").map(|n| isize::from_str(n).unwrap()).collect()
        ).collect();
    
    println!("Histories: {:?}", histories);

    let score : isize = histories.iter()
        .map(|h| turtles_all_the_way_down(h, false))
        .sum();

    println!("Part1 score: {score}");
}

fn part2(contents: &str) {
    let histories : Vec<Vec<isize>> = contents.lines()
        .map(|l|
            l.split(" ").map(|n| isize::from_str(n).unwrap()).collect()
        ).collect();

    println!("Histories: {:?}", histories);

    let score : isize = histories.iter()
        .map(|h| turtles_all_the_way_down(h, true))
        .sum();

    println!("Part1 score: {score}");
}

fn turtles_all_the_way_down(history: &Vec<isize>, prepend: bool) -> isize {
    let mut stack = vec![history.clone()];
    let mut source = history.clone();
    
    while !source.iter().all(|n| *n == 0) {
        source = source_sequence(&source);
        stack.push(source.clone());
    }

    let mut number = 0;
    for seq in stack.iter().rev().skip(1) {
        if prepend {
            number = seq.first().unwrap() - number
        } else {
            number = number + seq.last().unwrap()
        }
        // if prepend {
        //     println!("Current #: {}, {:?}", number, seq);
        // }
    }

    // println!("Depth: {}, last number: {}", stack.len(), last_number);
    println!("Current #: {}", number);
    // debug print
    // for seq in stack.iter() {
    //     println!("{:?}", seq)
    // }

    number
}

fn source_sequence(input: &Vec<isize>) -> Vec<isize> {
    input.windows(2)
        .map(|s| s[1]-s[0])
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        assert_eq!(source_sequence(&vec![0, 3, 6, 9, 12, 15]),
             vec![3, 3, 3, 3, 3]);
        assert_eq!(source_sequence(&vec![3, 3, 3, 3, 3]),
            vec![0, 0, 0, 0]);
    }
}