use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
};

use std::hash::{Hash, Hasher};

use anyhow::anyhow;
use nom::{
    bytes::complete::take_till,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};

pub fn digit_to_i32(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |d: &str| d.parse::<i32>())(input)
}

/// Parser uses parse_fn to parse the line and consumes whatever it doesn't parse
/// Should result in going to the next line.
pub fn parse_input_lines<F, T>(input: &str, parse_fn: F) -> anyhow::Result<(&str, Vec<T>)>
where
    F: Fn(&str) -> IResult<&str, T>,
{
    let parser = map(tuple((parse_fn, consume_line)), |(item, _)| item);
    many1(parser)(input).map_err(|err| anyhow!(err.to_string()))
}

/// Consume line and return remainder
pub fn consume_line(input: &str) -> IResult<&str, ()> {
    map(tuple((take_till(|c| c == '\n'), opt(line_ending))), |_| ())(input)
}

pub fn read_file(name: &str) -> Vec<String> {
    let mut buf = String::new();
    BufReader::new(File::open(name).unwrap()).read_to_string(&mut buf).unwrap();
    buf.split('\n').map(|s| s.to_string()).collect()
}

/// Point is (x, y)
pub type Point = (isize, isize);
pub type GridContents = HashMap<Point, char>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub contents: GridContents,
    pub width: isize,
    pub height: isize,
}

impl Hash for Grid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut contents = self.contents.iter().collect::<Vec<_>>();
        contents.sort();
        contents.hash(state);
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let lines = input.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut grid = Self {
            contents: HashMap::new(),
            width: lines[0].len() as isize,
            height: lines.len() as isize,
        };
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.contents.insert((x as isize, y as isize), c);
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    pub fn iterate_from_point(&self, start: Point, direction: Direction) -> GridIteratorDirectional {
        GridIteratorDirectional::new(self, start, direction)
    }
    pub fn iterate_elements(&self) -> GridIteratorAll {
        GridIteratorAll::new(self)
    }
}

pub struct GridIteratorDirectional<'a> {
    grid: &'a Grid,
    current: Point,
    direction: Direction,
}

impl<'a> GridIteratorDirectional<'a> {
    fn new(grid: &'a Grid, start: Point, direction: Direction) -> Self {
        Self { grid, current: start, direction }
    }
}

impl<'a> Iterator for GridIteratorDirectional<'a> {
    type Item = (Point, char);

    fn next(&mut self) -> Option<Self::Item> {
        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        self.current = (self.current.0 + dx, self.current.1 + dy);

        if self.current.0 < 0 || self.current.0 >= self.grid.width || self.current.1 < 0 || self.current.1 >= self.grid.height {
            None
        } else if let Some(&value) = self.grid.contents.get(&self.current) {
            Some((self.current, value))
        } else {
            self.next()
        }
    }
}

pub struct GridIteratorAll<'a> {
    grid: &'a Grid,
    current: Point,
}

impl<'a> GridIteratorAll<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self { grid, current: (0, 0) }
    }
}

impl<'a> Iterator for GridIteratorAll<'a> {
    type Item = (Point, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.0 >= self.grid.width {
            self.current.0 = 0;
            self.current.1 += 1;
        }

        if self.current.1 >= self.grid.height {
            return None;
        }

        let result = self.grid.contents.get(&self.current).map(|&value| (self.current, value));
        self.current.0 += 1;
        result
    }
}
