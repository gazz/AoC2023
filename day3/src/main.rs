use regex::Regex;
use std::cmp;
use std::env;
use std::fs;
use std::str::FromStr;
use std::ops::Range;
use itertools::Itertools;

const SYMBOL_MASK_RE: &'static str = r"[^\d\\.\n]";
const STAR_MASK_RE: &'static str = r"\*";

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part_one(&contents);

    part_two(&contents);
}

fn part_one(contents: &str) {
    let mask = symbol_mask(&contents, SYMBOL_MASK_RE);
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

fn part_two(contents: &str) {
    // iterate over numbers but save the star location this time

    let re: Regex = Regex::new(r"\d+").unwrap();
    let star_mask = symbol_mask(&contents, r"\*");
    let star_hits: Vec<(usize, Vec<(usize, usize)>)> = contents.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .enumerate()
        .map(|(y, line_str)| -> Vec<(usize, Vec<(usize, usize)>)> {
            re.captures_iter(line_str)
                .map(|c| -> (usize, Vec<(usize, usize)>) {
                    // check hit on mask with all the digit positions
                    let part_match = c.get(0).unwrap();
                    let nearby_stars = find_nearby_symbol_locations(&star_mask, y, &part_match.range());
                    let part_score = if nearby_stars.len() > 0 { usize::from_str(part_match.as_str()).unwrap() } else { 0 };
                    (part_score, nearby_stars)
                }).collect()
        })
        .flatten()
        .filter(|(part_score, _)| *part_score > 0)
        .collect();
    println!("Star hits: {:?}", star_hits);

    // group by star points
    let ratios_sum : usize = star_hits.iter()
        .sorted_by_key(|(_, b)| b)
        .group_by(|(_, b)| b)
        .into_iter()
        .map(|(k, group)| {
            let part_numbers: Vec<(usize, &Vec<(usize, usize)>)> = group.map(|(part_num, g)| {
                // part_num.clone()
                (part_num.clone(), g)
                // (), k.clone())
            }).collect();
            // println!("Part numbers to mult: {:?}", part_numbers);
            if part_numbers.len() < 2 { return 0 };
            part_numbers.iter().map(|(part_num, _)| part_num).product()
        }).sum();
    println!("Gear ratios sum: {:?}", ratios_sum);


}


fn symbol_mask(input:  &str, re: &str) -> Vec<Vec<usize>> {
    // find all symbol matches
    let re: Regex = Regex::new(re).unwrap();
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

        let exact_hit = if row_matches[y].1.contains(&x) { true } else { false };

        if exact_hit {
            return 2;
        }
        // println!("Testing {x}, {y}, with bounds: [{min_x}, {min_y}, {max_x}, {max_y}]");
        // try all 9 locations for a hit
        let position_score: usize = (min_y..max_y + 1).into_iter()
            .map(|test_y| {
                (min_x..max_x + 1).into_iter()
                    .map(|test_x| {
                        let res = if row_matches[test_y].1.contains(&test_x) { 1 } else { 0 };
                        // println!("Checking {test_x}, {test_y}, result: {res}");
                        res
                    }).sum::<usize>()
            })
            .sum();
        if position_score > 0 { 1 } else { 0 }
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
        vec![0, 0, 1, 2, 1, 2, 1, 0, 1, 1],
        vec![0, 0, 1, 2, 1, 1, 1, 1, 1, 2],
        vec![0, 0, 1, 1, 1, 1, 2, 1, 1, 1],
    ];
    assert_eq!(symbol_mask(input, SYMBOL_MASK_RE), output);
}

#[test]
fn test_symbol_mask_star() {
    let input = r#"
        ...$.*....
        ...*.....&
        ......#..."#;

    let output: Vec<Vec<usize>> = vec![
        vec![0, 0, 1, 1, 1, 2, 1, 0, 0, 0],
        vec![0, 0, 1, 2, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0],
    ];
    assert_eq!(symbol_mask(input, STAR_MASK_RE), output);
}

fn range_symbol_area_intersects(row: usize, range: &Range<usize>, symbol_x: usize, symbol_y: usize) -> bool {
    let min_range_x = if range.start == 0 { 0 } else { range.start - 1 };
    let min_row = if row == 0 { 0 } else { row - 1 };
    symbol_x >= min_range_x && symbol_x < range.end + 1
        && symbol_y >= min_row && symbol_y <= row + 1
}

#[test]
fn test_range_symbol_intersect() {
    // same row
    let test_range = 2..4;
    assert_eq!(range_symbol_area_intersects(1, &test_range, 0, 1), false);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 1, 1), true);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 3, 1), true);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 4, 1), true);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 5, 1), false);
    // above
    assert_eq!(range_symbol_area_intersects(1, &test_range, 0, 0), false);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 1, 0), true);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 3, 0), true);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 4, 0), true);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 5, 0), false);
    // below
    assert_eq!(range_symbol_area_intersects(1, &test_range, 0, 2), false);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 1, 2), true);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 3, 2), true);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 4, 2), true);
    assert_eq!(range_symbol_area_intersects(1, &test_range, 5, 2), false);
}

fn find_nearby_symbol_locations(mask: &Vec<Vec<usize>>, row: usize, range: &Range<usize>) -> Vec<(usize, usize)> {
    // check each star if its a match on range
    let (rows, cols) = (mask.len(), mask[0].len());

    let symbol_locations = 
        (0..rows).map(|y| {
            (0..cols).map(|x| -> Option<(usize, usize)> {
                if mask[y][x] == 2 {
                    return Some((x, y));
                }
                None
            })
            .filter(|r| r.is_some())
            .map(|star| star.unwrap())
            .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .filter(|(x, y)| 
            range_symbol_area_intersects(row, range, *x, *y)
        )
        .collect();

    println!("Symbol locations for range: {:?} on row {row} are: {:?}", range, symbol_locations);

    symbol_locations
}


#[test]
fn test_nearby_star_locations() {
    let mask = vec![
        vec![0, 0, 1, 1, 1, 2, 1, 0, 0, 0],
        vec![0, 0, 1, 2, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0],
    ];
    assert_eq!(find_nearby_symbol_locations(&mask, 0, &(0..1)), vec![] );
    assert_eq!(find_nearby_symbol_locations(&mask, 0, &(0..3)), vec![(3, 1)] );
    assert_eq!(find_nearby_symbol_locations(&mask, 1, &(4..5)), vec![(5, 0), (3, 1)] );
}