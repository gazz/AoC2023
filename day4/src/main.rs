use std::collections::HashSet;
use std::str::FromStr;
use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let in_mem_lines: Vec<&str> = contents.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .collect();

    part1(&in_mem_lines);
    part2(&in_mem_lines);
}

fn part1(lines: &Vec<&str>) {
    let winning_score : usize = lines.iter()
        .map(|l| parse_card(l))
        .map(|(set1, set2)| 
            set1.intersection(&set2).copied().collect::<HashSet<usize>>().len()
        )
        .map(|num_winners|
            if num_winners > 0 { 1 << num_winners - 1 } else { 0 }
        )
        .sum();
    // check the set intersect
    println!("Cards: {:?}", winning_score);
}

fn part2(lines: &Vec<&str>) {
    let total_cards = lines.len();
    let mut num_cards: Vec<usize> = vec![1; total_cards];

    lines.iter().map(|l| parse_card(l))
        .map(|(set1, set2)| 
            set1.intersection(&set2).copied().collect::<HashSet<usize>>().len()
        )
        .enumerate()
        .for_each(|(index, card_score)| {
            let count_of_cards = num_cards[index];
            for j in (index + 1)..(index + 1 + card_score) {
                if j < total_cards {
                   num_cards[j] += count_of_cards;
                }
            }
        });
    // println!("Debug winning cards: {:?}", num_cards);
    let end_total_cards : usize = num_cards.iter().sum();
    println!("Total cards: {:?}", end_total_cards);
}

fn parse_card(line: &str) -> (HashSet<usize>, HashSet<usize>) {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // discard everything up to 'Card xx: '
    let oi : Vec<HashSet<usize>> = line.split([':', '|'])
        .skip(1)
        .map(|s| {
            s.trim().split(" ")
                .filter(|s| !s.trim().is_empty())
                .map(|n| usize::from_str(n).unwrap())
                .collect::<HashSet<usize>>()
        })
        .collect();
    return (oi[0].clone(), oi[1].clone());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        assert_eq!(parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            (HashSet::from([41, 48, 83, 86, 17]),
            HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])));
    }

}

