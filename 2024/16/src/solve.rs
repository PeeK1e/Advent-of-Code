use std::{any::TypeId, cell::RefCell, collections::{HashMap, HashSet}, fmt, future::pending, hash::{Hash, Hasher}, i32, thread::panicking};


#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut chart = Chart::new(input);
    let unvisited: Vec<&Tile> = chart.tiles.iter().map(|row| row.iter().map(|col| col).collect::<Vec<&Tile>>()).flatten().collect();
    chart.update_distances(unvisited);

    let mut end = &chart.tiles[0][0];
    chart.tiles.iter().for_each(|row| {
        row.iter().for_each(|t| {if t.tile_type == TileType::End { end = t; return;}});
    });

    let mut start = &chart.tiles[0][0];
    chart.tiles.iter().for_each(|row| {
        row.iter().for_each(|t| {
            if t.tile_type == TileType::Start {
                start = t;
            return;
            }
        });
    });
    
    let path = chart.find_path(end, start);
    //chart.print(&path); 

    let count = *path.first().unwrap().distance.borrow() as i64; 
    Ok(count) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut chart = Chart::new(input);
    let unvisited: Vec<&Tile> = chart.tiles.iter().map(|row| row.iter().map(|col| col).collect::<Vec<&Tile>>()).flatten().collect();
    chart.update_distances(unvisited);

    // end tile
    let mut end = &chart.tiles[0][0];
    chart.tiles.iter().for_each(|row| {
        row.iter().for_each(|t| {if t.tile_type == TileType::End { end = t; return;}});
    });

    // start tile
    let mut start = &chart.tiles[0][0];
    chart.tiles.iter().for_each(|row| {
        row.iter().for_each(|t| {if t.tile_type == TileType::Start {start = t;return;}});
    });
    
    // this is our best path, we need to search all tiles that are a good path
    // this means, every tile which ich next to the main path needs its weight checked to see if multiple best paths exist
    let mut path = chart.find_path(end, start);
    path.remove(0);
    //chart.print(&path);

    let mut hset = HashSet::new();
    path.iter().for_each(|ele| {hset.insert(*ele);});

    loop {
        let mut newhset = hset.clone();
        for tile in hset.iter() {
            for newtile in chart.find_good_path_points(tile) {
                for foundtile in chart.find_path(newtile, start) {
                    newhset.insert(foundtile);
                }
            }
        }
        if hset.len() == newhset.len() {
            break;
        }
        hset = newhset;
    }

    let count = hset.len() + 1;

    Ok(count as i64) 
}

struct Chart {
    tiles: Vec<Vec<Tile>>,
    h: i32,
    w: i32
}

#[derive(PartialEq, Eq, Hash)]
enum TileType {
    Wall,
    Start,
    End,
    Floor
}

#[derive(PartialEq, Eq)]
struct Tile {
    tile_type: TileType,
    distance: RefCell<i32>,
    direction: RefCell<Direction>,
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NEUTRAL
}

impl Chart {
    fn new(input: &str) -> Self {
        let mut h = 0;
        let mut w = 0;
        let tiles: Vec<Vec<Tile>> = input
            .split("\n")
            .enumerate()
            .map(|(y, row)| {
                h = y;
                row
                    .chars()
                    .enumerate()
                    .map(|(x, ch)|{
                        w = x;
                        let tile_type = TileType::new(ch);
                        let distance = if tile_type == TileType::Start {
                            RefCell::new(0)
                        } else {
                            RefCell::new(i32::MAX)
                        };
                        Tile {
                            tile_type,
                            distance,
                            y,
                            x,
                            direction: RefCell::new(Direction::RIGHT)
                        }
                    })
                    .collect()
            })
            .collect();
            Chart{
                tiles,
                h: h as i32,
                w: w as i32
            }
    }
    fn update_distances(&self, unvisited: Vec<&Tile>) {
        let mut unvisited = unvisited;
        while unvisited.len() > 0 {
            // find current smallest distance
            let mut idx = 0;
            let mut dist = i32::MAX;
            unvisited
                .iter()
                .enumerate()
                .for_each(|(i, ele)|{
                    if *ele.distance.borrow() < dist {
                        dist = *ele.distance.borrow();
                        idx = i;
                    }
                });
            
            let tile = unvisited.get(idx).unwrap();
            if *tile.distance.borrow() != i32::MAX { 
                let (x,y) = (tile.x, tile.y);
                for (dx,dy) in [(1,0),(-1,0),(0,1),(0,-1)].iter() { 
                    if self.inbounds(x as i32 + dx, y as i32 + dy) {
                        if self.tiles[(y as i32 + dy) as usize][(x as i32 + dx) as usize].tile_type == TileType::Wall ||
                            self.tiles[(y as i32 + dy) as usize][(x as i32 + dx) as usize].tile_type == TileType::Start  {
                            continue;
                        }

                        let dir = Direction::new(*dx, *dy);
                        let current_tile_dis = *tile.distance.borrow();
                        let new_tile_dist = *self.tiles[(y as i32 + dy) as usize][(x as i32 + dx) as usize].distance.borrow();
                        let distance = current_tile_dis + tile.direction.borrow().get_weight(dir)+1;

                        // if new calculated dis is smaller than the old, replace it
                        if distance < new_tile_dist {
                            self.tiles[(y as i32 + dy) as usize][(x as i32 + dx) as usize].distance.replace(distance);
                            self.tiles[(y as i32 + dy) as usize][(x as i32 + dx) as usize].direction.replace(dir);
                        }
                    }
                }
            }
            unvisited.remove(idx);
        } 
    }

    fn find_path(&self, end: &Tile, start: &Tile) -> Vec<&Tile> {
        // find end tile
        let mut tile= &self.tiles[end.y][end.x];

        let mut path = vec![];
        path.push(tile);

        let (mut sx, mut sy) = (0,0);
        let mut dist = i32::MAX;
        while tile != start {
            // reverse path search
            let (x,y) = (tile.x, tile.y);
            for (dx,dy) in [(1,0),(-1,0),(0,1),(0,-1)].iter() {
                let (x,y) = (x as i32 + dx, y as i32 + dy);
                if self.inbounds(x,y) {
                    if self.tiles[y as usize][x as usize].tile_type == TileType::Wall ||
                        self.tiles[y as usize][x as usize].tile_type == TileType::End {
                        continue;
                    }

                    let tile_dist = *self.tiles[y as usize][x as usize]
                        .distance
                        .borrow();

                    if tile_dist < dist {
                        dist = tile_dist;
                        sx = x;
                        sy = y;
                    }
                }
            }
            tile = &self.tiles[sy as usize][sx as usize];
            path.push(&self.tiles[sy as usize][sx as usize]);
        } 
        path
    }

    fn print(&self, path: &Vec<&Tile>) {
        let mut map = self.get_str_map();
        for tile in path {
            let (x,y) = (tile.x, tile.y);
            if tile.tile_type != TileType::Start && tile.tile_type != TileType::End {
                map[y][x] = "O";
            }
        }
        map
            .iter()
            .for_each(|row| { 
                row
                    .iter()
                    .for_each(|col| print!("{col}"));
                println!();
            }
        );
    }

    fn inbounds(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        if x > self.h || y > self.w {
            return false;
        }
        return true;
    }

    fn get_str_map(&self) -> Vec<Vec<&str>> {
        self
            .tiles
            .iter()
            .map(|row| {
                row
                    .iter()
                    .map(|tile| {
                        tile.tile_type.char()
                    })
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<Vec<&str>>>()
    }

    fn find_good_path_points(&self, tile: &Tile) -> Vec<&Tile> {
        let mut tiles = vec![];
        for (dx,dy) in [(1,0),(-1,0),(0,1),(0,-1)].iter() {
            let (x,y) = (tile.x as i32 + dx, tile.y as i32 + dy);
            if self.inbounds(x,y) {
                let new_tile = &self.tiles[y as usize][x as usize];
                if new_tile.tile_type == TileType::Wall 
                    || new_tile.tile_type == TileType::Start 
                    || new_tile.tile_type == TileType::End {
                    continue;
                }

                // this indicates that we have a path of the same lenght, just some more or less turns
                if (*tile.distance.borrow()-1)%1000 == *new_tile.distance.borrow()%1000 {
                    tiles.push(&self.tiles[y as usize][x as usize]);
                }
            }
        }
        tiles
    }
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the fields individually
        self.tile_type.hash(state);
        self.x.hash(state);
        self.y.hash(state);
        // For `RefCell` fields, you can either skip or hash their inner values.
        // Be cautious with `borrow` to avoid panics.
        self.distance.borrow().hash(state);
        self.direction.borrow().hash(state);
    }
}

impl std::fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self
        .tiles
        .iter()
        .map(|row| {
            let r = row
                .iter()
                .map(|tile| {
                    tile.tile_type.char()
                })
                .collect::<Vec<&str>>();
            r.join("") + "\n"
        })
        .collect::<Vec<String>>().join("");
        write!(f, "{}", out)
    }
}

impl TileType {
    fn new(ch: char) -> Self {
        match ch {
            '.' => TileType::Floor,
            '#' => TileType::Wall,
            'E' => TileType::End,
            'S' => TileType::Start,
           _ => unreachable!() 
        }
    }

    fn char(&self) -> &str {
        match self {
            TileType::Floor => ".",
            TileType::Wall => "#",
            TileType::End => "E",
            TileType::Start => "S",
        }
    }
}

impl Direction {
    fn get_weight(&self, compare: Direction) -> i32 {
        if *self == Self::NEUTRAL {
            return 0;
        }
        if *self != compare {
            return 1000;
        } else {
            return 0;
        }
    }

    fn new(x: i32, y: i32) -> Self {
        match (x,y) {
            (1,0) => Self::RIGHT,
            (-1,0) => Self::LEFT,
            (0,1) => Self::DOWN,
            (0,-1) => Self::UP,
            (0,0) => Self::NEUTRAL,
            _ => unreachable!()
        }
    }
}