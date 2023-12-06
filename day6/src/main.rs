use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str) {
    let results = contents.lines()
        .map(|s| s.split(":").skip(1).next().unwrap())
        .map(|s| {
            s.trim().split(" ").filter(|s| !s.trim().is_empty())
                .map(|n| usize::from_str(n).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let paired = results[0].iter()
        .zip(results[1].iter())
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect::<Vec<(usize, usize)>>();

    println!("Results: {:?}", results);
    println!("Paired: {:?}", paired);
    
    let score = paired.iter()
        .map(|(a, b)| {
            let (start, end) = line_intersection(*a, *b);
            println!("Number of ways using input [{a}, {b}] : {}", end - start + 1);
            end - start + 1
        })
        .product::<usize>();
    println!("part 1 score is {score}");
}


fn part2(contents: &str) {
    let results = contents.lines()
        .map(|s| s.split(":").skip(1).next().unwrap())
        .map(|s| {
            usize::from_str(s.trim().replace(" ", "").trim()).unwrap()
        })
        .collect::<Vec<usize>>();

    println!("Results: {:?}", results);

    let (start, end) = line_intersection(results[0], results[1]);
    println!("part 2 score is {}", end - start + 1);
}

fn line_intersection(time: usize, record: usize) -> (usize, usize)  {
    // graph is y = x(7 - x) 
    // can be rewritten as -x^2 + 7x - 9 = 0 in form ax^2 + bx + c = 0
    // a = 1, b = time, c = record
    // discriminant = (b ** 2) - (4 * c)
    // solution 1: (-b - sqrt(discriminant) / 2
    // solution 2: (-b + sqrt(discriminant) / 2

    let discriminant: i64 = time as i64 * time as i64 - 4 * record as i64;
    let dis_sqrt = (discriminant as f64).sqrt();

    // let sol1 = (time - dis_sqrt as i64) / 2;
    // let sol2 = (time + dis_sqrt as i64) / 2;
    let sol1_f64 = (time as f64 - dis_sqrt) / 2.;
    let sol2_f64 = (time as f64 + dis_sqrt) / 2.;
    let sol1 = sol1_f64.floor() as i64;
    let sol2 = sol2_f64.ceil() as i64;
    println!("Input [{time}, {record}], i64 [{sol1}, {sol2}], f64 [{sol1_f64}, {sol2_f64}]");
    ((sol1 + 1) as usize, (sol2 - 1) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Time:      7  15   30
    // Distance:  9  40  200

    #[test]
    fn test_something() {
        assert_eq!(line_intersection(7, 9), (2, 5));
        assert_eq!(line_intersection(15, 40), (4, 11));
        assert_eq!(line_intersection(30, 200), (11, 19));
        assert_eq!(line_intersection(75, 1328), (29, 46));
        // assert_eq!(1, 0);

        assert_eq!(line_intersection(71530, 940200), (14, 71516));
    }
}