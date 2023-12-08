use std::env;
use std::fs;
use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    part1(&contents);
}


fn part1(contents: &str) {
    let mut hands : Vec<(&str, usize, usize)> = contents.lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            (parts[0], usize::from_str(parts[1]).unwrap(), hand_score(parts[0]))
        })
        .collect();
    hands.sort_by_key(|h| h.2);

    // println!("Sorted hands: {:?}", hands);
    
    let score = hands.iter().enumerate()
        .map(|(index, (h, bid, _))| {
            println!("Winnings of hand {}: {} * {} -> {}", h, index + 1, bid, bid * (index + 1));
            bid * (index + 1)
        })
        .sum::<usize>();

    println!("Part1 score: {score}");
}

#[derive(PartialEq, PartialOrd, Debug)]
#[repr(usize)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind   
}

fn char_pos_power(char_power: usize, position_power: usize) -> usize {
    (char_power + 1) * 13usize.pow(position_power as u32)
}

fn hand_score(hand: &str) -> usize {
    // come up with a unique score for the hand based on cards, 
    let card_scores  = vec!["A","K","Q","J","T","9","8","7","6","5","4","3","2"];
    let hand_type_score = (hand_type(hand) as usize) * 13usize.pow(6u32);

    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
    let card_pos_score = hand.chars().enumerate()
        .map(|(i, c)| {
            let position_power = 5 - i;
            let char_power = card_scores.iter().rev().position(|v| *v == c.to_string()).unwrap();
            
            let power = char_pos_power(char_power, position_power);
            println!("Char: {}, char index: {}, index_power: {} power: {}", 
                c, char_power, position_power, power);
    
            power
        })
        .sum::<usize>();

    println!("Hand: {}, TypeScore: {}, PosScore: {}, Returning score: {}", 
        hand, hand_type_score, card_pos_score, hand_type_score + card_pos_score);

    hand_type_score + card_pos_score
}

fn hand_type(hand: &str) -> HandType {
    // sort card by chars & then count how many dupes there are
    let mut chars: Vec<char> = hand.chars().collect();
    chars.sort();
    let mut char_runs = chars.iter()
        .map(|c| { (c, 1usize) })
        .coalesce(|(c, n), (d, m)| {
            if c == d { Ok((c, n + m)) } else { Err(((c, n), (d, m))) }})
        .map(|c_res| c_res.1)
        .filter(|c| *c > 1)
        .collect::<Vec<usize>>();
    char_runs.sort();


    if char_runs.len() == 0 { return HandType::HighCard };
    if char_runs.len() > 1 {
        // two pair or full house
        if char_runs[0] == 3 || char_runs[1] == 3 { 
            return HandType::FullHouse;
        };
        return HandType::TwoPair;
    }
    match char_runs[0] {
        2 => HandType::OnePair,
        3 => HandType::ThreeOfAKind,
        4 => HandType::FourOfAKind,
        _ => HandType::FiveOfAKind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_strength() {
        assert_eq!(hand_type("32T3K"), HandType::OnePair);
        assert_eq!(hand_type("KK677"), HandType::TwoPair);
        assert_eq!(hand_type("T55T5"), HandType::FullHouse);
        assert_eq!(hand_type("T52J3"), HandType::HighCard);
    }

    #[test]
    fn test_char_at_index_power() {
        // assert_eq!(char_pos_power(0, 5), 416);
        // assert_eq!(char_pos_power(13, 4), 221);

        let lp = char_pos_power(0, 5)
            + char_pos_power(0, 4)
            + char_pos_power(0, 3)
            + char_pos_power(0, 2)
            + char_pos_power(0, 1);
        let rp = char_pos_power(12, 1) 
            + char_pos_power(12, 2)
            + char_pos_power(12, 3)
            + char_pos_power(12, 4);
        
        println!("Left: {lp}, Right: {rp}");
        assert_eq!(lp > rp, true);
        assert_eq!(lp > rp, true);

        // assert_eq!(char_pos_power(0, 5) > char_pos_power(13, 1), true);
    }
}