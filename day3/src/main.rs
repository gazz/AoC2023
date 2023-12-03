use regex::Regex;
use std::cmp;

fn main() {
    println!("does nothing");
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

    println!("Matches per row: {:?}", row_matches);

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


// #[test]
// fn test_symbol_mask() {
//     let input = r#"
//         467..114..
//         ...*......
//         ..35..633.
//         ......#...
//         617*......
//         .....+.58.
//         ..592.....
//         ......755.
//         ...$.*....
//         .664.598.."#;

//     let output: Vec<Vec<usize>> = vec![
//         vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0,],
//         vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0,],
//         vec![0, 0, 1, 1, 1, 1, 1, 1, 0, 0,],
//         vec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0,],
//         vec![0, 0, 1, 1, 1, 1, 1, 1, 0, 0,],
//         vec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0,],
//         vec![0, 0, 1, 1, 1, 1, 1, 1, 0, 0,],
//         vec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0,],
//         vec![0, 0, 1, 1, 1, 1, 1, 1, 0, 0,],
//         vec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0,],
//     ];
//     assert_eq!(symbol_mask(input), output);
// }


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