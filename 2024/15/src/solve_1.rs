use std::{fmt, sync::RwLock};

#[allow(unused_variables,unused_mut)]
pub fn solve1(input: &str) -> Result<i64, String> {
    let mut count = 0;

    let mut dungeon = Dungeon::new(input);

    let mut directions = Direction::move_stack(input);

    // println!();
    // println!("{dungeon}");

    let (mut x, mut y) = dungeon.get_robot_pos();
    while directions.len() > 0 {
        let dir = directions.get(0).unwrap().clone();
        directions.remove(0);

        if dungeon.is_space_at_end_of_row(dir, x, y) {
            let tile = dungeon.get_tile_type(x as usize, y as usize);
            dungeon.move_tiles(dir, tile, x, y); 
            let (dx, yx) = dir.get_delta();
            x += dx;
            y += yx;
        }
    }

    // println!();
    // println!("{dungeon}");

    count = dungeon.get_box_coord_sum();
    
    Ok(count as i64) 
}

#[derive(Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Dungeon {
    tiles: Vec<Vec<Option<Tile>>>,
}

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Robot,
    Border,
    Box,
}

impl Dungeon {
    fn new(input: &str) -> Dungeon {
        let parsed: Vec<&str> = input
            .split("\n\n")
            .collect();

        let map: Vec<&str> = parsed[0].split("\n").collect();
        
        let mut size_y = 0;
        let mut size_x = 0;

        let tiles = map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                size_y = y;
                line
                    .chars()
                    .enumerate()
                    .map(|(x, ch)| {
                        size_x = x;
                        match ch {
                            '#' => Some(Tile::Border),
                            '@' => Some(Tile::Robot),
                            'O' => Some(Tile::Box),
                            _ => None
                        }
                    })
                    .collect::<Vec<Option<Tile>>>()
            })
            .collect::<Vec<Vec<Option<Tile>>>>();

        Dungeon{
            tiles,
        }
    }

    fn get_robot_pos(&self) -> (isize,isize) {
        for (y, cols) in self.tiles.iter().enumerate() {
            for (x, row) in cols.iter().enumerate() {
                if let Some(tile) = row {
                    if *tile == Tile::Robot {
                        return (x as isize, y as isize);
                    }
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

        if let Some(tile) = &self.tiles[y as usize][x as usize] {
            if *tile == Tile::Box {
                return self.is_space_at_end_of_row(direction, x, y);
            }
            if *tile == Tile::Border {
                return false;
            }
        }
        true
    }

    // get next tile
    // if empty, move current tile
    // if not empty, call self with next index
    fn move_tiles(&mut self, direction: Direction, tile: Tile, x: isize, y: isize) {
        let (dx, dy) = direction.get_delta();

        // if next tile not empty, go to next tile
        if let Some(tile) = &self.tiles[(y+dy) as usize][(x+dx) as usize] {
            self.move_tiles(direction, tile.clone(),x+dx, y+dy);
        }

        self.tiles[(y+dy) as usize][(x+dx) as usize] = Some(tile);
        self.tiles[(y) as usize][(x) as usize] = None;

        // let current_tile =  self.get_tile_type(x as usize, y as usize);
        // let next_tile = 
        
        // self.tiles[y as usize][x as usize] = None;

        // self.tiles[(y+dy) as usize][(x+dx) as usize] = Some(current_tile);
    }

    fn get_tile_type(&self, x: usize, y: usize) -> Tile {
        let tile = self.tiles[y][x].as_ref().unwrap();
        match tile {
            Tile::Border => return Tile::Border,
            Tile::Robot => return Tile::Robot,
            Tile::Box => return Tile::Box,
        }
    }
    
    fn get_box_coord_sum(&self) -> u64 {
        let mut sum = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let Some(tile) = col {
                    if *tile == Tile::Box {
                        sum += 100 * y + x;
                    }
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
                if let Some(tile) = row {
                    string = string.to_owned() + tile.char();
                } else {
                    string = string.to_owned() + ".";
                }
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
            Tile::Box => "O",
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
}