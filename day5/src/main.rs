use std::str::FromStr;
use std::env;
use std::fs;
use core::ops::Range;
use std::iter;
use std::cmp;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // part1(&contents);
    part2(&contents);

}

fn part1(contents: &str) {
    let sections : Vec<String> = parse_sections(contents);
    println!("Sections: {:?}", sections);

    let seeds: Vec<usize> = sections[0].split(":").skip(1)
        .map(|s| {
            s.trim().split(" ").filter(|s| !s.trim().is_empty())
                .map(|n| usize::from_str(n).unwrap())
                .collect::<Vec<usize>>()
        }).next().unwrap();

    println!("Seeds: {:?}", seeds);

    let ranges = sections.iter().skip(1)
        .map(|section| {
            let data_lines = section.lines().skip(1)
                .fold(String::new(), |s, l| s + l + "\n");
            parse_ranges(&data_lines)
        })
        .collect::<Vec<Vec<(Range<usize>, usize)>>>();

    println!("Ranges: {:?}", ranges);

    let locations = seeds.iter().map(|seed| {
        let mut lookup_id = *seed;
        for i in 0..ranges.len() {
            lookup_id = map_lookup(&ranges[i], lookup_id);
        }
        lookup_id
    }).collect::<Vec<usize>>();

    println!("Locations: {:?}, min location: {}", locations, locations.iter().min().unwrap());

}

fn part2(contents: &str) {
    let sections : Vec<String> = parse_sections(contents);
    // println!("Sections: {:?}", sections);

    let seeds = sections[0].split(":").skip(1)
        .map(|s| {
            let seeds_input = s.trim().split(" ").filter(|s| !s.trim().is_empty())
                .map(|n| usize::from_str(n).unwrap())
                .collect::<Vec<usize>>();
            let mut ranges = seeds_input.chunks(2).map(|a| {
                    a[0]..(a[0] + a[1])
                })
                .collect::<Vec<Range<usize>>>();
            ranges.sort_by_key(|r| r.start);
            ranges
        }).next().unwrap();

    println!("Seeds: {:?}", seeds);

    let ranges = sections.iter().skip(1)
        .map(|section| {
            let data_lines = section.lines().skip(1)
                .fold(String::new(), |s, l| s + l + "\n");
            parse_ranges(&data_lines)
        })
        .collect::<Vec<Vec<(Range<usize>, usize)>>>();

    let mut step_ranges = seeds;
    for step_mapping in ranges {
        
        let step_output = step_ranges.iter()
            .map(|step_range| {
                map_lookup_ranges(&step_mapping, step_range)
            })
            .flatten()
            .collect::<Vec<Range<usize>>>();
        
        println!("-- Step output: {:?}", step_output);
        step_ranges = step_output;
    }

    step_ranges.sort_by_key(|r| r.start);

    println!("Closest location is {}", step_ranges[0].start);

 }

fn parse_ranges(input: &str) -> Vec<(Range<usize>, usize)> {
    // println!("Parsing ranges: {input}");
    input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty()) 
        .map(|s| {
                s.trim().split(" ")
                    .filter(|s| !s.trim().is_empty())
                    .map(|n| usize::from_str(n).unwrap())
                    .collect::<Vec<usize>>()
        })
        .map(|n| {
            ((n[1])..(n[1] + n[2]), n[0])
        })
        .collect::<Vec<(Range<usize>, usize)>>()
}

fn map_lookup(mapping: &Vec<(Range<usize>, usize)>, index: usize) -> usize {
    match mapping.iter().find(|r| r.0.contains(&index)) {
        Some(r) => r.1 + index - r.0.start,
        None => index
    }
}

#[derive(Debug)]
struct RangeOverlap {
    leading: Option<Range<usize>>,
    matching: Option<Range<usize>>,
    trailing: Option<Range<usize>>,
}

fn map_lookup_ranges(mapping: &Vec<(Range<usize>, usize)>, in_range: &Range<usize>) -> Vec<Range<usize>> {
    // build ranges based on mapping (and create new ranges if they are out of mappings scope)
    let range_overlap_fn = |map_range: &(Range<usize>, usize), test_range: &Range<usize>| -> RangeOverlap {
        let out_of_overlap = test_range.start >= map_range.0.end ||
            test_range.end < map_range.0.start;


        if out_of_overlap {
            println!("No overlap with ranges: {:?} and {:?}", map_range, test_range);
            return RangeOverlap {
                leading: None,
                matching: None,
                trailing: Some(test_range.clone())
            };
        }

        println!("Building range overlap with ranges: {:?} and {:?}", map_range, test_range);

        let has_leading = test_range.start < map_range.0.start;
        let has_trailing = test_range.end > map_range.0.end;

        let start = if test_range.start >= map_range.0.start { 
            map_range.1 + test_range.start - map_range.0.start
        } else {
            map_range.1
        };
        let end = if test_range.end < map_range.0.end { 
            map_range.1 + test_range.end - map_range.0.start
        } else {
            map_range.1 + map_range.0.end - map_range.0.start
        };
        RangeOverlap { 
            leading: if has_leading { Some(test_range.start..map_range.0.start) } else { None },
            matching: Some(start..end), 
            trailing: if has_trailing { Some(map_range.0.end..test_range.end) } else { None },
        }
    };

    let mut all_ranges: Vec<Range<usize>> = Vec::new();
    let mut reminder_range = in_range.clone();
    let mut i = 0usize;
    loop {
        if i >= mapping.len() { 
            all_ranges.push(reminder_range);
            break; 
        }
        let overlap = range_overlap_fn(&mapping[i], &reminder_range);
        println!("Overlap result: {:?}, test_range: {:?}", overlap, reminder_range);

        if overlap.leading.is_some() { all_ranges.push(overlap.leading.unwrap()); }
        if overlap.matching.is_some() { all_ranges.push(overlap.matching.unwrap()); }
        if overlap.trailing.is_none() { break; }
        reminder_range = overlap.trailing.unwrap();
        i += 1;
    }

    println!("Ranges: {:?}", all_ranges);

    all_ranges.sort_by_key(|r| r.start);
    all_ranges
 }

fn parse_sections(input: &str) -> Vec<String> {
    let mut skip = 0usize;
    iter::repeat(0)
        .map_while(|_| {
            let output = input.lines()
                .skip(skip)
                .map_while(|l| {
                    skip += 1;
                    if l.trim().is_empty() { None }
                    else { Some(l.trim()) }
                })
                .fold(String::new(), |s, l| s + l + "\n");
            if output.is_empty() { None }
            else { Some(output) }
        })
        .collect::<Vec<String>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ranges() {
        let input = r"
            50 98 2
            52 50 48";

        let ranges = parse_ranges(input);
        assert_eq!(ranges, vec![(98..100, 50), (50..98, 52)]);
    }

    #[test]
    fn test_map_lookup() {
        assert_eq!(map_lookup(&vec![(98..100, 50)], 10), 10);
        assert_eq!(map_lookup(&vec![(98..100, 50)], 98), 50);
        assert_eq!(map_lookup(&vec![(98..100, 50)], 100), 100);
    }

    #[test]
    fn test_parse_sections() {
        let input = r"abc

        cde
        efg
        
        hij";

        let parsed: Vec<String> = parse_sections(input);

        println!("Sections: '{:?}'", parsed);

        assert_eq!(parsed[0], "abc\n");
        assert_eq!(parsed[1], "cde\nefg\n");
        assert_eq!(parsed[2], "hij\n");
    }

    #[test]
    fn test_map_lookup_ranges() {
        assert_eq!(map_lookup_ranges(&vec![(98..100, 50)], &(55..68)), vec![55..68]);

        assert_eq!(map_lookup_ranges(&vec![(98..100, 50)], &(0..12)), vec![0..12]);
        assert_eq!(map_lookup_ranges(&vec![(98..100, 50)], &(98..99)), vec![50..51]);
        assert_eq!(map_lookup_ranges(&vec![(98..100, 50)], &(98..102)), 
            vec![50..52, 100..102]);
        assert_eq!(map_lookup_ranges(&vec![(98..100, 50), (104..107, 30)], 
            &(95..110)), 
            vec![30..33, 50..52, 95..98, 100..104, 107..110]);

        assert_eq!(map_lookup_ranges(&vec![(50..98, 52)], &(55..68)), vec![57..70]);

        assert_eq!(map_lookup_ranges(&vec![(50..98, 52)], &(79..93)), vec![81..95]);

        assert_eq!(map_lookup_ranges(&vec![(52..54, 37)], &(81..95)), vec![81..95]);

        assert_eq!(map_lookup_ranges(&vec![(205465305..631361705, 0)], &(152560994..215857274)), 
            vec![0..10391969, 152560994..205465305]);
    }

}