use std::collections::HashMap;
use std::{str::FromStr, num::ParseIntError};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let red = usize::from_str(&args[2]).unwrap();
    let green = usize::from_str(&args[3]).unwrap();
    let blue = usize::from_str(&args[4]).unwrap();

    println!("Calculating possible games with red: {red}, green: {green}, blue: {blue}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let games : Vec<_> = contents.lines()
        .map(|s| s.trim())
        .filter(|&x| !x.is_empty())
        .map(|g| parse_game_stats(g).unwrap()).collect();

    let valid_game_score : usize = games.iter().map(|game| {
        for d in game.draws.iter() {
            if d.red > red || d.green > green || d.blue > blue {
                // println!("Game not possible {:?}", game);
                return 0;
            }
        }
        // println!("Game is possible {:?}", game);
        game.index
    }).sum::<usize>();

    println!("Games possible score: {valid_game_score}");

    // calculate the power of all games
    let power_sum : usize = games.iter().map(|game| {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for d in game.draws.iter() {
            max_red = if d.red > max_red { d.red } else { max_red };
            max_green = if d.green > max_green { d.green } else { max_green };
            max_blue = if d.blue > max_blue { d.blue } else { max_blue };
        }
        max_red * max_green * max_blue
    }).sum::<usize>();

    println!("Sum of game powers: {power_sum}");


}


#[derive(Debug)]
struct GameDraw {
    red: usize,
    green: usize,
    blue: usize
}

#[derive(Debug)]
struct Game {
    index: usize,
    draws: Vec<GameDraw>
}

fn parse_game_stats(s: &str) -> Option<Game> {
    // println!("Parsing game stats '{}'", s);
    match s.find(":") {
        None => None,
        Some(index) => {
            match (parse_game_index(&s[..index]), parse_game_draws(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some(Game {
                    index: l,
                    draws: r
                }),
                _ => None
            }
        }
   }
}

fn parse_game_index(s: &str) -> Result<usize, ParseIntError> {
    usize::from_str(s.split(" ").last().unwrap())
}

fn parse_game_draws(s: &str) -> Result<Vec<GameDraw>, ParseIntError> {
    // println!("parsing game draws: '{}'", s);

    Ok(s.split(";").map(|draw_str| {
        // println!("game draw: '{}'", draw_str);
        // let (num, color) = draw_str.split_at(draw_str.find(",").unwrap());
        let colors : HashMap<_, _> = draw_str.split(",")
            .map(|s| s.trim())
            .map(|color_str| {
                // println!("colors: '{}'", color_str);
                let (num, color) = color_str.trim().split_at(color_str.find(" ").unwrap());
                (color.trim(), usize::from_str(num).unwrap())
            }).into_iter().collect();

        // println!("draw: '{:?}'", colors);

        GameDraw {
            red: *colors.get("red").unwrap_or(&0),
            blue: *colors.get("blue").unwrap_or(&0),
            green: *colors.get("green").unwrap_or(&0)
        }
    }).collect())
}


#[test]
fn test_parse_game_index() {
    assert_eq!(parse_game_index("Game 23").unwrap(), 23);
    assert_eq!(parse_game_index("Game 2").unwrap(), 2);
    assert_eq!(parse_game_index("Game zzz").is_err(), true);
}

#[test]
fn test_parse_game_draws() {
    let input = r"3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let parsed = parse_game_draws(input).unwrap();
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0].red, 4);
    assert_eq!(parsed[0].blue, 3);
    assert_eq!(parsed[1].red, 1);
    assert_eq!(parsed[1].green, 2);
    assert_eq!(parsed[1].blue, 6);

    // error case
    // let r = parse_game_draws("3 blue, z red");
    // assert_eq!(parse_game_draws("3 blue, z red").is_err(), true)
}


#[test]
fn test_parse_game_stats() {

    let stats1 = parse_game_stats("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
    assert_eq!(stats1.index, 1);
    assert_eq!(stats1.draws.len(), 3);
    assert_eq!(stats1.draws[0].red, 4);

    let stats5 = parse_game_stats("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap();
    assert_eq!(stats5.index, 5);
    assert_eq!(stats5.draws.len(), 2);
    assert_eq!(stats5.draws[0].red, 6);


    let input = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;
    let games : Vec<_> = input.lines()
        .map(|s| s.trim())
        .filter(|&x| !x.is_empty())
        .map(|g| parse_game_stats(g).unwrap()).collect();
    assert_eq!(games.len(), 5);
}

