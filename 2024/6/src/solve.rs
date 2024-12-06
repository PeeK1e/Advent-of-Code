use std::{char, env::consts, error::Error, f32::DIGITS, usize};

const UP: &str = &"^";
const DOWN: &str = &"v";
const LEFT: &str = &"<";
const RIGHT: &str = &">";

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

type Map<'a> = Vec<Vec<&'a String>>;

struct Guard{
    Position: (i32,i32),
    Direction: Direction,
}

pub fn solve_t1(input: &str) -> Result<i32, String> {
    let mut count = 0;
    count += 1;

    let (map, guard) = make_map(input)?;

    Ok(count)
}

pub fn solve_t2(input: &str) -> i32 {
    let mut count = 0;
    count += 1;

    return count;
}

#[derive(Debug)]
enum SolveError {
    ParseError
}


fn make_map(input: &str) -> Result<(Vec<Vec<String>>, Guard), String> {
    let mut guard: Guard = Guard { Position: (0,0), Direction: Direction::RIGHT };

    let map = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .map(|(i,v)| {
            let mut line = vec![];
            for (pos, char) in v.as_bytes().iter().enumerate() {
                let mut char = char.to_string();
                if !(char == "." || char == "#"){
                    guard = Guard::new(pos, i, &char);
                    char = ".".to_string();
                }
                line.push(char);
            }
            line
        })
        .collect();

    Ok((map,guard))
}

impl Guard {
    fn new(x: usize, y: usize, direction: &str) -> Guard {
        let dir = Guard::parse_direction(direction);
        return Guard{ Position: (x as i32, y as i32), Direction: dir };
    }

    fn parse_direction(direction: &str) -> Direction {
        match direction {
            UP => Direction::UP,
            DOWN => Direction::DOWN,
            LEFT => Direction::LEFT,
            RIGHT => Direction::RIGHT,
            _ => Direction::UP
        }
    }
    
    fn rotate_dir(&mut self) {
        match self.Direction {
            Direction::UP =>  {self.Direction = Direction::RIGHT },
            Direction::RIGHT => {self.Direction = Direction::DOWN},
            Direction::DOWN => {self.Direction = Direction::LEFT},
            Direction::LEFT => {self.Direction = Direction::UP}
        }
    }
    
}