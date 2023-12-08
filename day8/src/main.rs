use std::collections::HashMap;
use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part_one(&contents);

    // part_two(&contents);
}

fn part_one(contents: &str) {
    // parse directions
    let mut directions = contents.lines().next().unwrap().chars().cycle();
    let nodes = parse_graph(contents.lines().skip(2));

    // start node
    let mut current_node = nodes.get("AAA").unwrap();
    let mut steps = 0usize;
    while current_node.name != "ZZZ" {
        steps += 1;
        let next_move = directions.next().unwrap();
        println!("Step {steps}: current node: {:?}, next move: {}", current_node, next_move);

        if next_move == 'L' {
            current_node = nodes.get(current_node.left).unwrap();
        } else {
            current_node = nodes.get(current_node.right).unwrap();
        }
    }
    println!("Took {steps} steps to reach destination ZZZ");
}


#[derive(Debug)]
struct Node<'a > {
    name: &'a str,
    left: &'a str,
    right: &'a str
}

fn parse_graph<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, Node<'a>> {
    let mut nodes = HashMap::new();
    let re: Regex = Regex::new(r"[A-Z]{3}").unwrap();

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