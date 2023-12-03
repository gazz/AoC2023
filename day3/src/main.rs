use regex::Regex;
use std::cmp;
use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mask = symbol_mask(&contents);

    let re: Regex = Regex::new(r"\d+").unwrap();

    let parts_sum = contents.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .enumerate()
        .map(|(y, line_str)| {
            re.captures_iter(line_str)
                .map(|c| -> usize {
                    // check hit on mask with all the digit positions
                    let part_match = c.get(0).unwrap();
                    let mask_hit = part_match.range().into_iter()
                        .map(|x| {
                            mask[y][x]
                        }).sum::<usize>();
                    if mask_hit > 0 { usize::from_str(part_match.as_str()).unwrap() } else { 0 }
                })
            .sum::<usize>()
        })
        .sum::<usize>();
    println!("Parts sum: {parts_sum}");
}


fn symbol_mask(input:  &str) -> Vec<Vec<usize>> {
    // find all symbol matches
    let re: Regex = Regex::new(r"[^\d\\.\n]").unwrap();
    let trimmed_lines : Vec<&str> = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .collect();
    let (num_lines, num_cols) = (trimmed_lines.len(), trimmed_lines[0].len());

    let row_matches : Vec<(usize, Vec<usize>)> = trimmed_lines.iter().enumerate()
        .map(|(i, l)| 
            // grab index as usize for each symbol on the given row
            (i, re.captures_iter(l)
                .map(|c| c.get(0).unwrap().start())
                .collect())
        )
        .collect();

    // println!("Matches per row: {:?}", row_matches);

    let hit_score = |x: usize, y: usize| -> usize {
        let min_x = if x == 0 { 0 } else { x - 1 };
        let min_y = if y == 0 { 0 } else { y - 1};
        let max_x = cmp::min(x + 1, num_cols - 1);
        let max_y = cmp::min(y + 1, num_lines - 1);

        // println!("Testing {x}, {y}, with bounds: [{min_x}, {min_y}, {max_x}, {max_y}]");
        // try all 9 locations for a hit
        (min_y..max_y + 1).into_iter()
            .map(|test_y| {
                (min_x..max_x + 1).into_iter()
                    .map(|test_x| {
                        let res = if row_matches[test_y].1.contains(&test_x) { 1 } else { 0 };
                        // println!("Checking {test_x}, {test_y}, result: {res}");
                        res
                    }).sum::<usize>()
            })
            .sum()
    };

    // build mask
    (0..num_lines).into_iter()
        .map(|y| 
            (0..num_cols).into_iter().map(|x| {
                hit_score(x, y)
            }).collect()
        )
        .collect()
}

#[test]
fn test_symbol_mask_small() {
    let input = r#"
        ...$.*....
        ...*.....&
        ......#..."#;

    let output: Vec<Vec<usize>> = vec![
        vec![0, 0, 2, 2, 3, 1, 1, 0, 1, 1],
        vec![0, 0, 2, 2, 3, 2, 2, 1, 1, 1],
        vec![0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    assert_eq!(symbol_mask(input), output);
}