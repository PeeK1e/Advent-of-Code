use std::{char, env::consts, error::Error, f32::DIGITS, str, usize};

const UP: &str = &"^";
const DOWN: &str = &"v";
const LEFT: &str = &"<";
const RIGHT: &str = &">";
const OBSTACLE: &str = &"#";

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

struct Map {
    Map: Vec<Vec<String>>,
}

struct Guard{
    // x, y coordinates
    Position: (i32, i32),
    Direction: Direction,
}

pub fn solve_t1(input: &str) -> Result<i32, String> {
    let mut count = 0;

    let map = Map::new(&input);
    let mut guard = Guard::new(&input);

    count = guard.patrol(map);

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

impl Map {
    fn new(input: &str) -> Map {
        let mut map = Map{Map: vec![vec![]]};
        let mapper = input
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| {
                let mut line = vec![];
                for char in v.chars() {
                    let mut char = char.to_string();
                    if !(char == "." || char == "#") {
                        char = ".".to_string();
                    }
                    line.push(char);
                }
                line
            })
            .collect();

        map.Map = mapper;
        map
    }

    fn set_visited(&mut self, x: i32, y: i32) {
        let row = self.Map.get_mut(y as usize).unwrap();
        let mut field = row.get_mut(x as usize).unwrap();

        *field = "X".to_string();
    }

    fn is_out_of_bounds(&self, x: i32, y: i32) -> bool {
        let y_bound = self.Map.len();
        let x_bound = self.Map.get(0).unwrap().len();
    
        if x < 0 || x >= x_bound as i32 {
            return true;
        }
        if y < 0 || y >= y_bound as i32 {
            return true;
        }
    
        return false;
    }

    fn visited_field_count(&self) -> i32 {
        self.Map
            .iter()
            .map(|line| {
                line
                    .iter()
                    .filter(|field| **field == "X")
                    .map(|_| 1)
                    .sum::<i32>()
        })
        .sum()
    }

    fn get_tile(&self, x: i32, y: i32) -> &str {
        let row = self.Map.get(y as usize).unwrap();
        row.get(x as usize).unwrap()
    }
}


impl Guard {
    fn new(input: &str) -> Guard {
        
        let (mut x, mut y, mut char) = (0,0,"".to_string());
        for (ypos, line) in input.split("\n").enumerate(){
            for (xpos, field ) in line.chars().enumerate() {
                let field = field.to_string();
                if field != "." && field != "#" {
                    x = xpos;
                    y = ypos;
                    char = field;
                }
            }
        };

        let dir = Guard::parse_direction(&char);
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

    fn patrol(&mut self, mut map: Map) -> i32 {
        loop {
            map.set_visited(self.Position.0, self.Position.1);

            let (x,y) = match self.Direction {
                Direction::UP => (self.Position.0, self.Position.1-1),
                Direction::RIGHT => (self.Position.0+1, self.Position.1),
                Direction::DOWN => (self.Position.0, self.Position.1+1),
                Direction::LEFT => (self.Position.0-1, self.Position.1)
            };
            if map.is_out_of_bounds(x, y) {
                break;
            }
            if map.get_tile(x, y) == OBSTACLE {
                self.rotate_dir();
                continue; 
            }
            self.Position = (x,y);
        }
        map.visited_field_count()
    }
    
}


