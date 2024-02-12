use std::{collections::HashMap, fmt::Display, fs::read_to_string};

/// Point is (x, y)
type Point = (i64, i64);
type GridContents = HashMap<Point, char>;

#[derive(Debug)]
struct Grid {
  contents: GridContents,
  width: i64,
  height: i64,
}

impl From<&str> for Grid {
  fn from(input: &str) -> Self {
    let lines = input.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    let mut grid = Self {
      contents: HashMap::new(),
      width: lines[0].len() as i64,
      height: lines.len() as i64,
    };
    for (y, line) in lines.iter().enumerate() {
      for (x, c) in line.chars().enumerate() {
        grid.contents.insert((x as i64, y as i64), c);
      }
    }
    grid
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = String::new();
    for y in 0..self.height {
      for x in 0..self.width {
        s.push(*self.contents.get(&(x, y)).unwrap());
      }
      s.push('\n');
    }
    write!(f, "{}", s)
  }
}

impl Grid {
  #[allow(dead_code)]
  fn surrounding_points(&self, point: &Point) -> Vec<Point> {
    (-1..=1)
      .flat_map(|dx| (-1..=1).map(move |dy| (point.0 + dx, point.1 + dy)))
      .filter(|p| p.0 < self.width && p.0 >= 0 && p.1 < self.height && p.1 >= 0)
      .collect()
  }
  fn valid_point(&self, p: &Point) -> bool {
    p.0 >= 0 && p.0 < self.width && p.1 >= 0 && p.1 < self.height
  }
}

#[derive(Debug)]
struct GalaxyGrid {
  grid: Grid,
  galaxy_points: Vec<(i64, i64)>,
}

impl GalaxyGrid {
  fn new(input: &str, spacetime_factor: i64) -> Self {
    let mut grid = GalaxyGrid {
      grid: Grid::from(input),
      galaxy_points: vec![],
    };
    grid.galaxy_points = grid.get_galaxy_points();
    grid.expand(spacetime_factor);
    grid
  }

  fn get(&self, p: &Point) -> &char {
    if self.grid.valid_point(p) {
      return self.grid.contents.get(p).unwrap();
    }
    panic!("invalid point: {:?}", p);
  }

  /// Calculates the distance between all the galaxies
  fn total_galaxy_distance(&self) -> i64 {
    let mut pairs = vec![];
    for i in 0..self.galaxy_points.len() {
      for j in i + 1..self.galaxy_points.len() {
        pairs.push((self.galaxy_points[i], self.galaxy_points[j]))
      }
    }
    pairs.iter().map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs()).sum()
  }

  /// Returns a vector with Y coordinates for rows without a galaxy
  fn empty_rows(&self) -> Vec<i64> {
    let mut empty_rows: Vec<i64> = vec![];
    'y: for y in 0..self.grid.height {
      for x in 0..self.grid.width {
        if *self.get(&(x, y)) != '.' {
          continue 'y;
        }
      }
      empty_rows.push(y);
    }
    empty_rows
  }

  /// Returns a vector with X coordinates for columns without a galaxy
  fn empty_cols(&self) -> Vec<i64> {
    let mut empty_cols: Vec<i64> = vec![];
    'x: for x in 0..self.grid.width {
      for y in 0..self.grid.height {
        if *self.get(&(x, y)) != '.' {
          continue 'x;
        }
      }
      empty_cols.push(x);
    }
    empty_cols
  }

  /// Expands the universe by factor and updates galaxy locations accordingly
  fn expand(&mut self, factor: i64) {
    let mut empty_rows = self.empty_rows();
    let mut empty_cols = self.empty_cols();
    empty_rows.push(i64::MAX);
    empty_cols.push(i64::MAX);
    for gp in self.galaxy_points.iter_mut() {
      for (i, x) in empty_cols.windows(2).enumerate() {
        if gp.0 > x[0] && gp.0 < x[1] {
          gp.0 += (factor - 1) * (i as i64 + 1);
          break;
        }
      }
      for (i, y) in empty_rows.windows(2).enumerate() {
        if gp.1 > y[0] && gp.1 < y[1] {
          gp.1 += (factor - 1) * (i as i64 + 1);
          break;
        }
      }
    }
  }

  fn get_galaxy_points(&self) -> Vec<Point> {
    let mut gp = self
      .grid
      .contents
      .iter()
      .filter(|(_, &v)| v == '#')
      .map(|(&point, _)| point)
      .collect::<Vec<_>>();
    gp.sort_by(|a, b| match a.0.cmp(&b.0) {
      std::cmp::Ordering::Equal => a.1.cmp(&b.1),
      o => o,
    });
    gp
  }
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let grid = GalaxyGrid::new(input.as_str(), 2);
  println!("Solution 1: {}", grid.total_galaxy_distance());
  let grid = GalaxyGrid::new(input.as_str(), 1000000);
  println!("{:?}", grid);
  println!("Solution 2: {}", grid.total_galaxy_distance());
}
