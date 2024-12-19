use std::{cell::RefCell, fmt, hash::{Hash, Hasher}, i32::{self}};


#[allow(unused_variables,unused_mut)]
pub fn solve_t1(input: &str, x: i32, y: i32, limit: i32) -> Result<i64, String> {
    let mut chart = Chart::new(input, x, y,limit);

    let unvisited: Vec<&Tile> = chart.tiles.iter().map(|row| row.iter().map(|col| col).collect::<Vec<&Tile>>()).flatten().collect(); 
    chart.update_distances(unvisited);

    let mut start = &chart.tiles[0][0];
    let mut end = &chart.tiles[(y-1) as usize][(x-1) as usize];
    
    let path = chart.find_path(end, start);

    let count = *path.first().unwrap().distance.borrow() as i64;

    Ok(count)
}

#[allow(unused_variables,unused_mut)]
pub fn solve_t2(input: &str, x: i32, y: i32, limit: i32) -> Result<(usize,usize), String> {
    let coords = input.split("\n").map(|coord|{
        let xy = coord.split(",").collect::<Vec<&str>>();
        let x = xy[0].parse::<usize>().unwrap();
        let y = xy[1].parse::<usize>().unwrap();
        (x,y)
    })
    .collect::<Vec<(usize,usize)>>();
    
    let mut i = limit;
    loop {
        //println!("Row: {}", i);
        let mut chart = Chart::new(input, x, y, i);
        let unvisited: Vec<&Tile> = chart.tiles.iter().map(|row| row.iter().map(|col| col).collect::<Vec<&Tile>>()).flatten().collect(); 
        chart.update_distances(unvisited);
    
        let mut start = &chart.tiles[0][0];
        let mut end = &chart.tiles[(y-1) as usize][(x-1) as usize];
        
        //let path = chart.find_path(end, start);
    
        //chart.print_path(&path)

        if *chart.tiles[(y-1) as usize][(x-1) as usize].distance.borrow() == i32::MAX {
            break;
        }

        i+=1;
    }
    let ele = coords[(i-1) as usize];
    Ok(ele)
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
    x: usize,
    y: usize,
}


impl Chart {
    fn new(input: &str, x: i32, y: i32, limit: i32) -> Self {
        let mut chart = vec![];
        for y in 0..y {
            let mut row = vec![];
            for x in 0..x {
                let t = Tile{
                    tile_type: TileType::Floor,
                    distance: RefCell::new(i32::MAX),
                    y: y as usize,
                    x: x as usize,
                };
                row.push(t);
            }
            chart.push(row);
        } 

        let mut i = 0; 
        input.split("\n").for_each(|coord|{
            let xy = coord.split(",").collect::<Vec<&str>>();
            let x = xy[0].parse::<usize>().unwrap();
            let y = xy[1].parse::<usize>().unwrap();  
            if i < limit {
                chart[y][x] = Tile{
                    tile_type: TileType::Wall,
                    distance: RefCell::new(i32::MAX),
                    y: y as usize,
                    x: x as usize,
                };
            }
            i += 1;
        }); 

        chart[0][0] = Tile{
            tile_type: TileType::Start,
            distance: RefCell::new(0),
            y: 0 as usize,
            x: 0 as usize,
        };
        chart[(y-1) as usize][(x-1) as usize] = Tile{
            tile_type: TileType::End,
            distance: RefCell::new(i32::MAX),
            y: (y-1) as usize,
            x: (x-1) as usize,
        };

        let chart = Chart{
            tiles: chart,
            h: y-1,
            w: x-1,
        };
        chart
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

                        let current_tile_dis = *tile.distance.borrow();
                        let new_tile_dist = *self.tiles[(y as i32 + dy) as usize][(x as i32 + dx) as usize].distance.borrow();
                        let distance = current_tile_dis + 1;

                        // if new calculated dis is smaller than the old, replace it
                        if distance < new_tile_dist {
                            self.tiles[(y as i32 + dy) as usize][(x as i32 + dx) as usize].distance.replace(distance);
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


    fn print(&self) {
        let map = self.get_str_map();
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

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the fields individually
        self.tile_type.hash(state);
        self.x.hash(state);
        self.y.hash(state);
        // For `RefCell` fields, you can either skip or hash their inner values.
        // Be cautious with `borrow` to avoid panics.
        self.distance.borrow().hash(state);
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
    fn char(&self) -> &str {
        match self {
            TileType::Floor => ".",
            TileType::Wall => "#",
            TileType::End => "E",
            TileType::Start => "S",
        }
    }
}
