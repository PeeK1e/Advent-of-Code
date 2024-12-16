use std::{fmt, sync::RwLock};

#[allow(unused_variables,unused_mut)]
pub fn solve2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    let mut dungeon = Dungeon::new(input);

    let mut directions = Direction::move_stack(input);

    while directions.len() > 0 {
        let (x, y) = dungeon.get_robot_pos();
        let dir = directions.get(0).unwrap().clone();
        directions.remove(0);

        if dungeon.is_space_at_end_of_row(dir, x as isize, y as isize) {
            let tile = dungeon.get_tile_type(x, y);
            dungeon.move_tiles(dir, tile, x as isize, y as isize); 
        }
    }

    count = dungeon.get_box_coord_sum();
    
    Ok(count as i64) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    Ok(count) 
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Dungeon {
    tiles: Vec<Vec<Tile>>,
}

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Robot,
    Border,
    BoxLeft,
    BoxRight,
    None
}

impl Dungeon {
    fn new(input: &str) -> Dungeon {
        let parsed: Vec<&str> = input
            .split("\n\n")
            .collect();

        let map: Vec<&str> = parsed[0].split("\n").collect();

        let mut tiles = vec![];

        for cols in map.iter() {
            let mut row = vec![];
            for ch in cols.chars() {
                match ch {
                    '.' => {
                        row.push(Tile::None);
                        row.push(Tile::None);
                    },
                    '#' => {
                        row.push(Tile::Border);
                        row.push(Tile::Border);
                    },
                    '@' => {
                        row.push(Tile::Robot);
                        row.push(Tile::None);
                    },
                    'O' => {
                        row.push(Tile::BoxLeft);
                        row.push(Tile::BoxRight)
                    },
                    _ => {}
                }
            }
            tiles.push(row);
        }

        Dungeon{
            tiles,
        }
    }

    fn get_robot_pos(&self) -> (usize,usize) {
        for (y, cols) in self.tiles.iter().enumerate() {
            for (x, row) in cols.iter().enumerate() {
                if *row == Tile::Robot {
                    return (x as usize, y as usize);
                }
            }
        }
        (0,0)
    }

    // this searches the row down for a tile
    // if it found nothing on a tile, return the position indicating a free slot
    fn is_space_at_end_of_row(&self, direction: Direction, x: isize, y: isize) -> bool {
        let (dx, dy) = direction.get_delta();
        let (x, y) = ((x + dx), (y + dy));

        let tile= &self.tiles[y as usize][x as usize];
        if direction == Direction::UP || direction == Direction::DOWN {
            if *tile == Tile::BoxLeft {
                if !self.is_space_at_end_of_row(direction, x, y) {
                    return false;
                }
                if !self.is_space_at_end_of_row(direction, x+1, y) {
                    return false;
                }
            } else if *tile == Tile::BoxRight  {
                if !self.is_space_at_end_of_row(direction, x, y) {
                    return false;
                }
                if !self.is_space_at_end_of_row(direction, x-1, y) {
                    return false;
                }
            }
        } else {
            if *tile == Tile::BoxLeft || *tile == Tile::BoxRight {
                return self.is_space_at_end_of_row(direction, x, y);
            } 
        }

        if *tile == Tile::Border {
            return false;
        }

        return true;
    }

    // get next tile
    // if empty, move current tile
    // if not empty, call self with next index
    fn move_tiles(&mut self, direction: Direction, tile: Tile, x: isize, y: isize) { 
        let (dx, dy) = direction.get_delta(); 

        let next_tile = self.get_tile_type((x+dx) as usize, (y+dy) as usize);
        if direction == Direction::UP || direction == Direction::DOWN {
            // vertically we need to move everything above us
            if next_tile == Tile::BoxLeft {
                self.move_tiles(direction, Tile::BoxLeft,x+dx, y+dy);   
                self.move_tiles(direction, Tile::BoxRight,x+dx+1, y+dy);
            } else if next_tile == Tile::BoxRight {
                self.move_tiles(direction, Tile::BoxRight,x+dx, y+dy);
                self.move_tiles(direction, Tile::BoxLeft,x+dx-1, y+dy);
            }
        } else {
            // horizontally we can move normally since were moving everything in the col anyway
            // if next tile not empty, go to next tile
            if next_tile == Tile::BoxLeft || next_tile == Tile::BoxRight {
                self.move_tiles(direction, next_tile.clone(),x+dx, y+dy);
            }
        }

        self.tiles[(y+dy) as usize][(x+dx) as usize] = tile;
        self.tiles[(y) as usize][(x) as usize] = Tile::None;
    }

    // returns a Tile of the type wanted
    fn get_tile_type(&self, x: usize, y: usize) -> Tile {
        match self.tiles[y][x] {
            Tile::Border => return Tile::Border,
            Tile::Robot => return Tile::Robot,
            Tile::BoxLeft => return Tile::BoxLeft,
            Tile::BoxRight => return Tile::BoxRight,
            Tile::None => return Tile::None,
        }
    }
    
    fn get_box_coord_sum(&self) -> u64 {
        let mut sum = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                    if *col == Tile::BoxLeft {
                        sum += 100 * y + x;
                    }
            }
        }
        sum as u64
    }
}

impl fmt::Display for Dungeon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_string();

        for col in self.tiles.iter() {
            for row in col {
                string = string.to_owned() + row.char();
            }
            string = string.to_owned() + "\n";
        }
        
        write!(f, "{}", string)
    }
}

impl Tile {
    fn char(&self) -> &str {
        match self {
            Tile::Robot => "@",
            Tile::Border => "#",
            Tile::BoxLeft => "[",
            Tile::BoxRight => "]",
            Tile::None => "."
        }
    }
}

impl Direction {
    fn move_stack(input: &str) -> Vec<Direction> {
        let moves: Vec<&str> = input.split("\n\n").collect();
        moves[1].replace("\n", "").chars().map(|ch| {
            match ch {
                '^' => Direction::UP,
                'v' => Direction::DOWN,
                '>' => Direction::RIGHT,
                _ => Direction::LEFT, 
            }
        })
        .collect::<Vec<Direction>>()
    }

    // returns the delta corresponding to the direction (x,y)
    fn get_delta(&self) -> (isize, isize) {
        match self {
            Direction::UP => (0,-1),
            Direction::LEFT => (-1,0),
            Direction::RIGHT => (1,0),
            Direction::DOWN => (0,1),
        }
    }
    fn char(&self) -> &str {
        match self {
            Direction::UP => "^",
            Direction::DOWN => "v",
            Direction::LEFT => "<",
            Direction::RIGHT => ">",
        }
    }
}