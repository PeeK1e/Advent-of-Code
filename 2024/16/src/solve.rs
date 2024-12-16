use std::{cell::RefCell, fmt, i32};


#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str) -> Result<i64, String> {
    let mut chart = Chart::new(input);
    println!("{chart}");

    let unvisited: Vec<&Tile> = chart.tiles.iter().map(|row| row.iter().map(|col| col).collect::<Vec<&Tile>>()).flatten().collect();
    chart.update_distances(unvisited);

    let path = chart.find_path();
    chart.print(&path); 

    let count = *path.first().unwrap().distance.borrow() as i64; 
    Ok(count) 
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str) -> Result<i64, String> {
    let mut count = 0;

    Ok(count) 
}

struct Chart {
    tiles: Vec<Vec<Tile>>,
    h: i32,
    w: i32
}

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq, Clone, Copy)]
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
                            direction: RefCell::new(Direction::NEUTRAL)
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

    fn find_path(&self) -> Vec<&Tile> {
        // find end tile
        let mut tile= &self.tiles[0][0];
        self.tiles.iter().for_each(|row| {
            row.iter().for_each(|t| {if t.tile_type == TileType::End { tile = t; return;}});
        });

        let mut path = vec![];
        path.push(tile);

        let (mut sx, mut sy) = (0,0);
        let mut dist = i32::MAX;
        while tile.tile_type != TileType::Start {
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
                map[y][x] = "%";
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