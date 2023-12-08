use std::collections::HashMap;
use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // part1(&contents);
    part2(&contents);
}

fn part1(contents: &str) {
    // parse directions
    let directions = contents.lines().next().unwrap();
    let nodes = parse_graph(contents.lines().skip(2));

    // start node
    let start_node = nodes.get("AAA").unwrap();
    let (distance, _) = find_distance(&nodes, start_node, directions);
    println!("Part 1 took {distance} steps to reach destination ZZZ");
}

fn find_distance<'a>(nodes: &'a HashMap<&str, Node>, start_node: &'a Node, dir: &str) -> (usize, &'a str) {
    let mut directions = dir.chars().cycle();
    let mut current_node = start_node;
    let mut steps = 0usize;
    let mut traverse_from_start = true;
    while traverse_from_start
        || !current_node.name.ends_with("Z") {
        traverse_from_start = false;
        steps += 1;
        let next_move = directions.next().unwrap();
        // println!("Step {steps}: current node: {:?}, next move: {}", current_node, next_move);

        if next_move == 'L' {
            current_node = nodes.get(current_node.left).unwrap();
        } else {
            current_node = nodes.get(current_node.right).unwrap();
        }
    }
    println!("Start Node: {}, End Node: {}, steps: {}", 
        start_node.name, current_node.name, steps);
    (steps, &current_node.name)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn part2(contents: &str) {
    let directions = contents.lines().next().unwrap();
    let nodes = parse_graph(contents.lines().skip(2));

    let traverse_distances = nodes.iter()
        .filter(|(k,v)| k.ends_with("A"))
        .map(|(_, v)| {
            find_distance(&nodes, v, directions)
        })
        .collect::<Vec<(usize, &str)>>();

    let traverse_distances2 = traverse_distances.iter()
        .map(|(_, v)| {
            find_distance(&nodes, nodes.get(v).unwrap(), directions)
        })
        .collect::<Vec<(usize, &str)>>();

    println!("Traverse disances1: {:?}", traverse_distances);
    // loop each path again
    println!("Traverse disances2: {:?}", traverse_distances2);

    // looks like each path is stuck in the same loop and ends up on 
    // the same node running the same distance every loop
    // eg - "GPA" reaches "LLZ" in 21979 steps, "LLZ" loops to itself in another 21979 steps
    // the result is a number that divides with each path distance
    // -> Lowest Common Multiple

    let distances = traverse_distances.iter()
        .map(|(d, _)| d.clone()).collect::<Vec<usize>>();
    let result: usize = lcm(&distances[0..]);

    println!("Result: {:?}", result);
    // println!("Part 2 took {steps} steps to reach all destinations ending with Z");

}

#[derive(Debug, PartialEq)]
struct Node<'a > {
    name: &'a str,
    left: &'a str,
    right: &'a str
}

fn parse_graph<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, Node<'a>> {
    let mut nodes = HashMap::new();
    let re: Regex = Regex::new(r"[A-Z0-9]{3}").unwrap();

    lines.map(|s| {
            let captures = re.captures_iter(s
                ).map(|c| c.get(0).unwrap().as_str())
                .collect::<Vec<&str>>();
            (captures[0], captures[1], captures[2])
        })
        .for_each(|(parent, left, right)| {
            nodes.insert(parent, Node { name: parent, left, right });
        });

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_graph() {

        let input = r"AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)";

        let nodes = parse_graph(input.lines());
        assert_eq!(nodes.get("AAA").unwrap().left, "BBB");
        assert_eq!(nodes.get("AAA").unwrap().right, "BBB");
        assert_eq!(nodes.get("BBB").unwrap().left, "AAA");
        assert_eq!(nodes.get("BBB").unwrap().right, "ZZZ");
        assert_eq!(nodes.get("ZZZ").unwrap().left, "ZZZ");
        assert_eq!(nodes.get("ZZZ").unwrap().right, "ZZZ");
    }

}