use std::{fmt, str, usize};

const UP: &str = &"^";
const DOWN: &str = &"v";
const LEFT: &str = &"<";
const RIGHT: &str = &">";
const OBSTACLE: &str = &"#";
const CUSTOM_OBSTACLE: &str = &"*";

#[derive(Clone,PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

type Previous = (i32,i32);
type Next = (i32,i32);

#[derive(Clone)]
struct Map {
    map: Vec<Vec<String>>,
    // Patroled Path
    // This will be used to "backtrace" the path the path the guard is taking and try to make loops out of it
}

#[derive(Clone)]
struct Guard{
    // x, y coordinates
    position: (i32, i32),
    direction: Direction,
    patroled_path: Vec<(Previous,Next, Direction)>,
}

pub fn solve_t1(input: &str) -> Result<i32, String> {
    let map = Map::new(&input);
    let mut guard = Guard::new(&input);

    let count = guard.patrol(map);

    Ok(count)
}

pub fn solve_t2(input: &str) -> Result<i32, String> {
    let mut count = 0;
    let map = Map::new(input);
    let clean_map = map.clone();
    let mut guard = Guard::new(input);
    let clean_guard = guard.clone();

    guard.patrol(map);
    let mut set_obstables = vec![];
    for (guard_pos, obstacle_pos, dir) in guard.patroled_path.clone() {
        if set_obstables.contains(&obstacle_pos) {
            continue;
        }


        let mut generate_map = clean_map.clone();
        generate_map.set_tile_as_obstacle(obstacle_pos.0, obstacle_pos.1);

        if clean_guard.clone().is_loopable(generate_map, guard_pos.0, guard_pos.1, dir) {
            set_obstables.push(obstacle_pos);
            count += 1;
        }
    }

    Ok(count)
}

impl Map {
    fn new(input: &str) -> Map {
        let mut map = Map{map: vec![vec![]]};
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

        map.map = mapper;
        map
    }

    fn set_visited(&mut self, x: i32, y: i32) {
        let row = self.map.get_mut(y as usize).unwrap();
        let field = row.get_mut(x as usize).unwrap();

        *field = "X".to_string();
    }

    fn is_out_of_bounds(&self, x: i32, y: i32) -> bool {
        let y_bound = self.map.len();
        let x_bound = self.map.get(0).unwrap().len();
    
        if x < 0 || x >= x_bound as i32 {
            return true;
        }
        if y < 0 || y >= y_bound as i32 {
            return true;
        }
    
        return false;
    }

    fn visited_field_count(&self) -> i32 {
        self.map
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

    fn is_visited(&self, x: i32, y: i32) -> bool {
        let row = self.map.get(y as usize).unwrap();
        let tile = row.get(x as usize).unwrap();
        *tile == "X"
    }

    fn get_tile(&self, x: i32, y: i32) -> &str {
        let row = self.map.get(y as usize).unwrap();
        row.get(x as usize).unwrap()
    }

    fn set_tile_as_obstacle(&mut self, x: i32, y: i32) {
        let row = self.map.get_mut(y as usize).unwrap();
        let tile = row.get_mut(x as usize).unwrap();
        *tile = CUSTOM_OBSTACLE.to_string();
    }

}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut string = "".to_string();
        string = string.to_owned() + " 0123456789";
        let mut count = 0;
        for line in &self.map {
            string = string.to_owned() + "\n" + &count.to_string();
            for tile in line {
                string = string.to_owned() + &tile.to_string();
            }
            count+=1;
        }
        write!(f, "{}", &string)
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
        return Guard{ position: (x as i32, y as i32), direction: dir , patroled_path: vec![]};
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
        match self.direction {
            Direction::UP =>  {self.direction = Direction::RIGHT },
            Direction::RIGHT => {self.direction = Direction::DOWN},
            Direction::DOWN => {self.direction = Direction::LEFT},
            Direction::LEFT => {self.direction = Direction::UP}
        }
    }

    fn patrol(&mut self, mut map: Map) -> i32 {
        loop {
            let (vx, vy, vd) = (self.position.0, self.position.1, self.direction.clone());
            map.set_visited (vx,vy);

            let (x,y) = match self.direction {
                Direction::UP => (self.position.0, self.position.1-1),
                Direction::RIGHT => (self.position.0+1, self.position.1),
                Direction::DOWN => (self.position.0, self.position.1+1),
                Direction::LEFT => (self.position.0-1, self.position.1)
            };
            if map.is_out_of_bounds(x, y) {
                break;
            }
            
            if map.get_tile(x, y) == OBSTACLE {
                self.rotate_dir();
                continue; 
            }
            self.patroled_path.push(((vx,vy),(x,y),vd));
            self.position = (x,y);
        }
        map.visited_field_count()
    }

    fn is_loopable(&mut self, mut map: Map, target_x: i32, target_y: i32, target_direction: Direction) -> bool {
       let mut bump = 0; 
        loop {
            let (vx, vy, vd) = (self.position.0, self.position.1, self.direction.clone());
            let (x,y) = match self.direction {
                Direction::UP => (self.position.0, self.position.1-1),
                Direction::RIGHT => (self.position.0+1, self.position.1),
                Direction::DOWN => (self.position.0, self.position.1+1),
                Direction::LEFT => (self.position.0-1, self.position.1)
            };
            if map.is_out_of_bounds(x, y) {
                return false;
            }

            let tile = map.get_tile(x, y);
            if tile == OBSTACLE || tile == CUSTOM_OBSTACLE {
                if map.is_visited(vx, vy) {
                    bump += 1;
                    if bump >= 10 {
                        // println!("{map}"); 
                        return true;
                    }
                }
                self.rotate_dir();
                continue; 
            }

            map.set_visited (vx,vy);
            self.patroled_path.push(((vx,vy),(x,y),vd));
            self.position = (x,y);
        }
    }    
}


