use std::{collections::HashSet, fs::read_to_string, ops::Rem};

#[derive(Debug, Clone)]
enum Turn {
  Left,
  Right,
  Straight,
}

#[derive(Debug, Clone)]
struct Coordinate {
  x: i32,
  y: i32,
}

#[derive(Debug, Clone)]
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
enum Direction {
  North,
  East,
  South,
  West,
  Same,
}

#[derive(Debug, Clone)]
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

  /// Check if this tile has another direction due to it being a corner/bend.
  ///
  /// When a tile has a direction, and it's a bend, it has to have another direction.
  /// For example, going into a 7 from the West, means the direction would be East (going from West
  /// to East). However, the direction is also South, because you're going through the 7,
  /// Southwards.
  fn bend_direction(&self, other: &CoordinateTile) -> Option<Direction> {
    let direction = other.direction(self);
    match self.tile {
      Tile::NorthEast => match direction {
        Direction::South => Some(Direction::East),
        Direction::West => Some(Direction::North),
        _ => unreachable!(),
      },
      Tile::NorthWest => match direction {
        Direction::South => Some(Direction::West),
        Direction::East => Some(Direction::North),
        _ => unreachable!(),
      },
      Tile::SouthWest => match direction {
        Direction::North => Some(Direction::West),
        Direction::East => Some(Direction::South),
        _ => unreachable!(),
      },
      Tile::SouthEast => match direction {
        Direction::North => Some(Direction::East),
        Direction::West => Some(Direction::South),
        _ => unreachable!(),
      },
      _ => None,
    }
  }

  /// Checks if the other CoordinateTile is N/E/S/W of this tile
  fn direction(&self, other: &CoordinateTile) -> Direction {
    if other.coordinate.x - self.coordinate.x >= 1 {
      Direction::East
    } else if other.coordinate.x - self.coordinate.x <= -1 {
      Direction::West
    } else if other.coordinate.y - self.coordinate.y <= -1 {
      Direction::North
    } else if other.coordinate.y - self.coordinate.y >= 1 {
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

  /// Returns whether the current tile takes a left or right turn, or continues straight ahead
  fn tile_turn(&self, prev: &CoordinateTile) -> Turn {
    match self.direction(prev) {
      Direction::North => match self.tile {
        Tile::NorthSouth => Turn::Straight,
        Tile::NorthWest => Turn::Right,
        Tile::NorthEast => Turn::Left,
        _ => panic!("wtf"),
      },
      Direction::East => match self.tile {
        Tile::EastWest => Turn::Straight,
        Tile::NorthEast => Turn::Right,
        Tile::SouthEast => Turn::Left,
        _ => panic!("wtf"),
      },
      Direction::South => match self.tile {
        Tile::NorthSouth => Turn::Straight,
        Tile::SouthEast => Turn::Right,
        Tile::SouthWest => Turn::Left,
        _ => panic!("wtf"),
      },
      Direction::West => match self.tile {
        Tile::EastWest => Turn::Straight,
        Tile::SouthWest => Turn::Right,
        Tile::NorthWest => Turn::Left,
        _ => panic!("wtf"),
      },
      Direction::Same => panic!("double you tee eff"),
    }
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
        Tile::NorthEast => self.new_tile_by_direction(grid, Direction::North),
        Tile::SouthEast => self.new_tile_by_direction(grid, Direction::South),
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
fn walk_count(
  grid: &Grid,
  pipe_count: usize,
  starting_point: &CoordinateTile,
  prev: &CoordinateTile,
  cur: &CoordinateTile,
) -> usize {
  let next_tile = cur.next_tile(grid, prev);
  if next_tile.same(starting_point) {
    return pipe_count;
  }
  walk_count(grid, pipe_count + 1, starting_point, cur, &next_tile)
}

fn walk_vec(
  grid: &Grid,
  visited_tiles: &mut Vec<CoordinateTile>,
  starting_point: &CoordinateTile,
  prev: &CoordinateTile,
  cur: &CoordinateTile,
) {
  visited_tiles.push(cur.clone());
  let next_tile = cur.next_tile(grid, prev);
  if next_tile.same(starting_point) {
    return;
  }
  walk_vec(grid, visited_tiles, starting_point, cur, &next_tile);
}

/// Find out the loop direction to know where to search for enclosed items
/// This is done by checking if there are more right or more left turns.
fn pipeline_direction(pipeline: &Vec<(CoordinateTile, Turn, Vec<Direction>)>) -> Turn {
  match pipeline.iter().fold(0, |acc, (_, t, _)| {
    acc
      + match t {
        Turn::Right => 1,
        Turn::Left => -1,
        _ => 0,
      }
  }) {
    x if x > 0 => Turn::Right,
    _ => Turn::Left,
  }
}

type PipelineEnhanced = Vec<(CoordinateTile, Turn, Vec<Direction>)>;
type Pipeline = Vec<CoordinateTile>;
/// Enrich the tiles we found while walking with:
/// - the turn that each tile makes
/// - the direction(s) each tile has
fn enhance_pipeline(pipeline: &Pipeline) -> PipelineEnhanced {
  let first = pipeline[0].clone();
  let mut pipeline_enhanced = pipeline
    .windows(2)
    .map(|tile_window| {
      let mut directions = vec![tile_window[0].direction(&tile_window[1])];
      if let Some(d) = tile_window[1].bend_direction(&tile_window[0]) {
        directions.push(d)
      };
      (tile_window[1].clone(), tile_window[1].tile_turn(&tile_window[0]), directions)
    })
    .collect::<Vec<_>>();
  pipeline_enhanced.insert(0, (first, Turn::Straight, vec![Direction::Same]));
  pipeline_enhanced
}

/// With all this information, we will search for enclosed fuggers.
/// We do this by iterating over each tile and looking towards the inside of the loop
/// Example:
///   The loop is right-sided.
///   We are facing north so we will walk East until we hit a loop pipe.
///   If we are bending west afterwards, through a 7, we look East and North from that 7.
fn find_enclosed_fuggers(pipeline_enhanced: &PipelineEnhanced, pipeline_direction: Turn) -> HashSet<(i32, i32)> {
  let max_pipeline_coordinate = pipeline_enhanced.iter().fold(0, |acc, (tile, _, _)| {
    acc.max(tile.coordinate.x.abs().max(tile.coordinate.y.abs()))
  });
  let mut enclosed_fuggers: HashSet<(i32, i32)> = HashSet::new();
  for (coordinate_tile, _, directions) in pipeline_enhanced.iter() {
    let offsets = directions
      .iter()
      .map(|direction| {
        let (dx, dy) = match direction {
          Direction::North => (1, 0),
          Direction::East => (0, 1),
          Direction::South => (-1, 0),
          Direction::West => (0, -1),
          Direction::Same => (0, 0),
        };
        if let Turn::Right = pipeline_direction {
          (dx, dy)
        } else {
          (-dx, -dy)
        }
      })
      .collect::<Vec<_>>();
    for offset in offsets.iter() {
      // For each tile's offset(s) we walk until we hit a loop-pipe
      for i in 1..max_pipeline_coordinate {
        let x = coordinate_tile.coordinate.x + offset.0 * i;
        let y = coordinate_tile.coordinate.y + offset.1 * i;
        if pipeline_enhanced
          .iter()
          .any(|(vt, _, _)| vt.coordinate.x == x && vt.coordinate.y == y)
        {
          break;
        }
        enclosed_fuggers.insert((x, y));
      }
    }
  }
  enclosed_fuggers
}

fn solution_two(grid: &Grid) {
  let start = grid.find_tile_by_char('S');
  let starting_pipe = start.find_starting_tile(grid);
  // Get the whole pipeline in a Vec
  let mut pipeline: Vec<CoordinateTile> = vec![start.clone()];
  walk_vec(grid, &mut pipeline, &start, &start, &starting_pipe);
  let pipeline_enhanced = enhance_pipeline(&pipeline);
  let pipeline_direction = pipeline_direction(&pipeline_enhanced);
  // println!("Loop direction: {:?}", loop_direction);
  let enclosed_fuggers = find_enclosed_fuggers(&pipeline_enhanced, pipeline_direction);

  println!("solution 2: {}", enclosed_fuggers.len());
}

fn main() {
  let input = read_to_string("input.txt.1").unwrap();
  let grid = Grid::new(&input);
  let start = grid.find_tile_by_char('S');
  let starting_pipe = start.find_starting_tile(&grid);
  let pipe_count = walk_count(&grid, 2, &start, &start, &starting_pipe);
  println!("solution 1: {:?}", pipe_count / 2);
  solution_two(&grid);
}
