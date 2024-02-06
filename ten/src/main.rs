use std::{fs::read_to_string, ops::Rem};

#[derive(Debug, Clone)]
struct Coordinate {
  x: i32,
  y: i32,
}

#[derive(Debug)]
enum Tile {
  NorthSouth,
  EastWest,
  NorthEast,
  NorthWest,
  SouthWest,
  SouthEast,
  Start,
  Ground,
}

impl Tile {
  fn from(c: &char) -> Self {
    match c {
      '|' => Self::NorthSouth,
      '-' => Self::EastWest,
      'L' => Self::NorthEast,
      'J' => Self::NorthWest,
      '7' => Self::SouthWest,
      'F' => Self::SouthEast,
      'S' => Self::Start,
      '.' => Self::Ground,
      _ => panic!("Invalid character for Tile"),
    }
  }
}

#[derive(Debug)]
struct CoordinateTile {
  coordinate: Coordinate,
  tile: Tile,
}

impl CoordinateTile {
  fn new(grid: &Grid, coordinate: &Coordinate) -> CoordinateTile {
    let tile = Tile::from(&grid.get_char_at_coord(coordinate));
    let coordinate = coordinate.clone();
    CoordinateTile { tile, coordinate }
  }

  fn same(&self, other: &CoordinateTile) -> bool {
    self.coordinate.x == other.coordinate.x && self.coordinate.y == other.coordinate.y
  }

  /// Generate the CoordinateTiles around this one
  fn surrounding_tiles(&self, grid: &Grid) -> Vec<CoordinateTile> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
      .iter()
      .map(|(dx, dy)| Coordinate {
        x: self.coordinate.x + dx,
        y: self.coordinate.y + dy,
      })
      .filter(|c| grid.valid_coordinate(&c))
      .map(|c| CoordinateTile::new(grid, &c))
      .collect()
  }

  /// Looks at which tile we are, and where we came from.
  /// Based on that, return the next tile
  fn next_tile(&self, grid: &Grid, prev: &CoordinateTile) -> CoordinateTile {
    // Prev is East
    if prev.coordinate.x - self.coordinate.x == 1 {
      match self.tile {
        Tile::EastWest => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x - 1,
              y: self.coordinate.y,
            },
          )
        }
        Tile::SouthEast => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x,
              y: self.coordinate.y + 1,
            },
          )
        }
        Tile::NorthEast => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x,
              y: self.coordinate.y - 1,
            },
          )
        }
        _ => panic!("invalid coordinate passed to next_coordinate"),
      }
    }
    // Prev is West
    if prev.coordinate.x - self.coordinate.x == -1 {
      match self.tile {
        Tile::EastWest => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x + 1,
              y: self.coordinate.y,
            },
          )
        }
        Tile::NorthWest => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x,
              y: self.coordinate.y - 1,
            },
          )
        }
        Tile::SouthWest => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x,
              y: self.coordinate.y + 1,
            },
          )
        }
        _ => panic!("invalid coordinate passed to next_coordinate"),
      }
    }
    // Prev is North
    if prev.coordinate.y - self.coordinate.y == -1 {
      match self.tile {
        Tile::NorthSouth => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x,
              y: self.coordinate.y + 1,
            },
          )
        }
        Tile::NorthWest => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x - 1,
              y: self.coordinate.y,
            },
          )
        }
        Tile::NorthEast => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x + 1,
              y: self.coordinate.y,
            },
          )
        }
        _ => panic!("invalid coordinate passed to next_coordinate"),
      }
    }
    // Prev is South
    if prev.coordinate.y - self.coordinate.y == 1 {
      match self.tile {
        Tile::NorthSouth => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x,
              y: self.coordinate.y - 1,
            },
          )
        }
        Tile::SouthEast => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x + 1,
              y: self.coordinate.y,
            },
          )
        }
        Tile::SouthWest => {
          return CoordinateTile::new(
            grid,
            &Coordinate {
              x: self.coordinate.x - 1,
              y: self.coordinate.y,
            },
          )
        }
        _ => panic!("invalid coordinate passed to next_coordinate"),
      }
    }
    panic!("wtf");
  }

  fn connected(&self, other: &CoordinateTile) -> bool {
    // Other is East
    if other.coordinate.x - self.coordinate.x == 1 {
      match other.tile {
        Tile::EastWest | Tile::SouthWest | Tile::NorthWest => return true,
        _ => return false,
      }
    }
    // Other is West
    if other.coordinate.x - self.coordinate.x == -1 {
      match other.tile {
        Tile::EastWest | Tile::SouthEast | Tile::NorthEast => return true,
        _ => return false,
      }
    }
    // Other is North
    if other.coordinate.y - self.coordinate.y == -1 {
      match other.tile {
        Tile::SouthWest | Tile::SouthEast | Tile::NorthSouth => return true,
        _ => return false,
      }
    }
    // Other is South
    if other.coordinate.y - self.coordinate.y == 1 {
      match other.tile {
        Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest => return true,
        _ => return false,
      }
    }
    false
  }

  /// Looks at surrounding tiles to see which one is connected
  /// and returns the first one, to start walking through the pipes
  fn find_starting_tile(&self, grid: &Grid) -> CoordinateTile {
    if let Tile::Start = self.tile {
      let surrounding = self.surrounding_tiles(grid);
      surrounding.into_iter().find(|s| self.connected(s)).unwrap()
    } else {
      panic!("Can't find starting point from self: {:?}", self.tile);
    }
  }
}

#[derive(Debug)]
struct Grid {
  contents: Vec<char>,
  width: i32,
  height: i32,
}

impl Grid {
  fn new(input: &str) -> Self {
    let mut grid = Grid {
      contents: input.chars().filter(|&x| x != '\n').collect(),
      width: input.chars().take_while(|&x| x != '\n').count() as i32,
      height: 0,
    };
    grid.height = grid.contents.len() as i32 / grid.width;
    grid
  }

  fn valid_coordinate(&self, coordinate: &Coordinate) -> bool {
    coordinate.x >= 0 && coordinate.x < self.width && coordinate.y >= 0 && coordinate.y < self.height
  }

  fn get_char_at_coord(&self, c: &Coordinate) -> char {
    if !self.valid_coordinate(c) {
      panic!("can't access matrix contents at coord: {:?}", c)
    }
    let idx = (c.y * self.width + c.x) as usize;
    self.contents[idx]
  }

  fn find_tile_by_char(&self, c: char) -> CoordinateTile {
    let (i, c) = self.contents.iter().enumerate().find(|(_, &x)| x == c).unwrap();
    let i = i as i32;
    let coordinate = Coordinate {
      x: i.rem(self.width),
      y: i / self.width,
    };
    let tile = Tile::from(c);
    CoordinateTile { coordinate, tile }
  }
}

/// Walks through the pipes and returns the amount of pipes encountered
fn walk(grid: &Grid, pipes: usize, starting_point: &CoordinateTile, prev: &CoordinateTile, cur: &CoordinateTile) -> usize {
  let next = cur.next_tile(&grid, prev);
  // println!("pipes: {}, prev: {:?}, cur: {:?}, next: {:?}", pipes, prev, cur, next);
  if next.same(starting_point) {
    return pipes;
  }
  walk(grid, pipes + 1, starting_point, cur, &next)
}

fn main() {
  let input = read_to_string("input.txt.real").unwrap();
  let grid = Grid::new(&input);
  let start = grid.find_tile_by_char('S');
  let starting_pipe = start.find_starting_tile(&grid);
  let pipes = walk(&grid, 2, &start, &start, &starting_pipe);
  println!("{:?}", pipes);
  println!("solution 1: {:?}", pipes / 2);
}
