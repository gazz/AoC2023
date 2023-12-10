use std::str::FromStr;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1(&contents);
}


fn part1(contents: &str) {
    // first idea is to brute force follow the loop to find its length length & divide by 2
    let tile_grid: Vec<Vec<char>> = contents.lines()
        .map(|l| l.chars().collect()).collect();

    // find start position 
    let mut start_index_ = None;
    let row_len = tile_grid[0].len();
    for i in 0..tile_grid.len() {
        for j in 0..row_len {
            if tile_grid[j][i] == 'S' {
                println!("Found start index: {}; {}", j, i);
                start_index_ = Some((j, i));
            }
        }
    }
    let start_location = start_index_.unwrap();
    println!("Start location: {:?}", start_location);


    // lets do 10k iterations as limit
    let mut prev_move = Move::None;
    let mut next_location = start_location;
    let mut steps = 0;
    for i in 0..1000000 {
        steps = i + 1;
        (next_location, prev_move) = traverse(&tile_grid, 
            &next_location, &prev_move);
        if next_location == start_location {
            break;
        }
    }

    println!("Traverse distance: {:?}, furthest tile: {}", steps, steps / 2);
}

#[derive(PartialEq, Debug)]
enum Move {
    North, East, South, West, None
}

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
fn legal_moves(grid: &Vec<Vec<char>>, pos: &(usize, usize)) -> (Move, Move) {
    // each tile has only 2 legal moves
    let tile = grid[pos.1][pos.0];
    match tile {
        'S' => {
            // just grabbed it from the input
            // (Move::East, Move::South)
            (Move::West, Move::South)
        },
        '|' => (Move::North, Move::South),
        '-' => (Move::East, Move::West),
        'L' => (Move::North, Move::East),
        'J' => (Move::North, Move::West),
        '7' => (Move::West, Move::South),
        'F' => (Move::East, Move::South),
        _ => (Move::None, Move::None)
    }
}

fn inverse_move(r#move : &Move) -> Move {
    match r#move {
        Move::North => Move::South,
        Move::South => Move::North,
        Move::East => Move::West,
        Move::West => Move::East,
        _ => Move::None
    }
}

// .....
// .S-7.
// .|.|.
// .L-J.
// .....

fn traverse(grid: &Vec<Vec<char>>, start_pos: &(usize, usize),
     from_move: &Move) -> ((usize, usize), Move) {
    // recursively traverse until we hit the start pos again & return the distance sum
    let legal_moves = legal_moves(grid, start_pos);
    
    // prevent going back the same direction
    let next_move = if legal_moves.0 != *from_move {
        &legal_moves.0
    } else { &legal_moves.1 };


    println!("Traversing from {:?} ({}) in direction: {:?} coming from dir: {:?}. legal moves: {:?}",
        start_pos, grid[start_pos.1][start_pos.0], next_move, from_move,
        legal_moves);

    assert_ne!(next_move, &Move::None);
    assert_ne!(next_move, from_move);
    
    
    let next_location = match next_move {
        Move::North => (start_pos.0, start_pos.1 - 1),
        Move::East => (start_pos.0 + 1, start_pos.1),
        Move::South => (start_pos.0, start_pos.1 + 1),
        Move::West => (start_pos.0 - 1, start_pos.1),
        _ => {
            start_pos.clone()
        }
    };
    
    (next_location, inverse_move(next_move))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        assert_eq!(true, false);
    }
}