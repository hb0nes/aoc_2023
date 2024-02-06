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

enum Direction {
  North,
  East,
  South,
  West,
  Same,
}

#[derive(Debug)]
struct CoordinateTile {
  coordinate: Coordinate,
  tile: Tile,
}

impl CoordinateTile {
  fn new(grid: &Grid, coordinate: &Coordinate) -> Self {
    let tile = Tile::from(&grid.get_char_at_coord(coordinate));
    let coordinate = coordinate.clone();
    Self { tile, coordinate }
  }

  /// Checks if this CoordinateTile is the same as another based on coordinates
  fn same(&self, other: &CoordinateTile) -> bool {
    self.coordinate.x == other.coordinate.x && self.coordinate.y == other.coordinate.y
  }

  /// Checks if the other CoordinateTile is N/E/S/W of this tile
  fn direction(&self, other: &CoordinateTile) -> Direction {
    if other.coordinate.x - self.coordinate.x == 1 {
      Direction::East
    } else if other.coordinate.x - self.coordinate.x == -1 {
      Direction::West
    } else if other.coordinate.y - self.coordinate.y == -1 {
      Direction::North
    } else if other.coordinate.y - self.coordinate.y == 1 {
      Direction::South
    } else {
      Direction::Same
    }
  }

  /// Generate the CoordinateTiles around this one
  fn surrounding_tiles(&self, grid: &Grid) -> Vec<CoordinateTile> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
      .iter()
      .map(|(dx, dy)| Coordinate {
        x: self.coordinate.x + dx,
        y: self.coordinate.y + dy,
      })
      .filter(|c| grid.valid_coordinate(c))
      .map(|c| CoordinateTile::new(grid, &c))
      .collect()
  }

  /// Return a new tile N/E/S/W of current tile by looking at the grid
  fn new_tile_by_direction(&self, grid: &Grid, direction: Direction) -> CoordinateTile {
    let (dx, dy) = match direction {
      Direction::North => (0, -1),
      Direction::East => (1, 0),
      Direction::South => (0, 1),
      Direction::West => (-1, 0),
      Direction::Same => panic!("Same for new tile..?"),
    };
    let coordinate = &Coordinate {
      x: self.coordinate.x + dx,
      y: self.coordinate.y + dy,
    };
    CoordinateTile::new(grid, coordinate)
  }

  /// Look at current tile and where the previous tile is.
  /// Based on that, return the next tile.
  /// For example:
  ///   - previous is West
  ///   - current tile is J
  ///   - Next tile is North
  fn next_tile(&self, grid: &Grid, prev: &CoordinateTile) -> CoordinateTile {
    match self.direction(prev) {
      Direction::North => match self.tile {
        Tile::NorthSouth => self.new_tile_by_direction(grid, Direction::South),
        Tile::NorthWest => self.new_tile_by_direction(grid, Direction::West),
        Tile::NorthEast => self.new_tile_by_direction(grid, Direction::East),
        _ => panic!("invalid coordinate passed to next_coordinate"),
      },
      Direction::East => match self.tile {
        Tile::EastWest => self.new_tile_by_direction(grid, Direction::West),
        Tile::SouthEast => self.new_tile_by_direction(grid, Direction::South),
        Tile::NorthEast => self.new_tile_by_direction(grid, Direction::North),
        _ => panic!("invalid coordinate passed to next_coordinate"),
      },
      Direction::South => match self.tile {
        Tile::NorthSouth => self.new_tile_by_direction(grid, Direction::North),
        Tile::SouthEast => self.new_tile_by_direction(grid, Direction::East),
        Tile::SouthWest => self.new_tile_by_direction(grid, Direction::West),
        _ => panic!("invalid coordinate passed to next_coordinate"),
      },
      Direction::West => match self.tile {
        Tile::EastWest => self.new_tile_by_direction(grid, Direction::East),
        Tile::NorthWest => self.new_tile_by_direction(grid, Direction::North),
        Tile::SouthWest => self.new_tile_by_direction(grid, Direction::South),
        _ => panic!("invalid coordinate passed to next_coordinate"),
      },
      Direction::Same => panic!("double you tee eff"),
    }
  }

  /// Checks if other CoordinateTile is connected to this one
  /// For example, if Other is | and self is -, Other is not connected.
  fn connected(&self, other: &CoordinateTile) -> bool {
    match self.direction(other) {
      Direction::North => matches!(other.tile, Tile::SouthWest | Tile::SouthEast | Tile::NorthSouth),
      Direction::East => matches!(other.tile, Tile::EastWest | Tile::SouthWest | Tile::NorthWest),
      Direction::South => matches!(other.tile, Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest),
      Direction::West => matches!(other.tile, Tile::EastWest | Tile::SouthEast | Tile::NorthEast),
      Direction::Same => {
        panic!("double you tee eff")
      }
    }
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
fn walk(grid: &Grid, pipe_count: usize, starting_point: &CoordinateTile, prev: &CoordinateTile, cur: &CoordinateTile) -> usize {
  let next = cur.next_tile(grid, prev);
  if next.same(starting_point) {
    return pipe_count;
  }
  walk(grid, pipe_count + 1, starting_point, cur, &next)
}

fn main() {
  let input = read_to_string("input.txt.real").unwrap();
  let grid = Grid::new(&input);
  let start = grid.find_tile_by_char('S');
  let starting_pipe = start.find_starting_tile(&grid);
  let pipe_count = walk(&grid, 2, &start, &start, &starting_pipe);
  println!("solution 1: {:?}", pipes / 2);
}
