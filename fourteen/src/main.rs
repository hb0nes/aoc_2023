use std::{collections::HashMap, time::Instant};

use aoclib::{Direction, Grid};
use itertools::Itertools;

enum Rock {
    Empty,
    Round,
    Square,
}

impl Rock {
    fn from_char(c: &char) -> Rock {
        match c {
            '.' => Rock::Empty,
            'O' => Rock::Round,
            '#' => Rock::Square,
            _ => panic!("Invalid character"),
        }
    }
}

/// Receives a point anywhere in the grid and a direction
/// Returns the point on the edge of the grid in a given direction, starting at point
fn point_edge(grid: &Grid, point: aoclib::Point, direction: &aoclib::Direction) -> aoclib::Point {
    match direction {
        aoclib::Direction::Left => (0, point.1),
        aoclib::Direction::Right => (grid.width - 1, point.1),
        aoclib::Direction::Up => (point.0, 0),
        aoclib::Direction::Down => (point.0, grid.height - 1),
    }
}

/// Receives a point that should contain a solid piece of rock and a direction
/// Returns the point before that
fn point_before(point: aoclib::Point, direction: &aoclib::Direction) -> aoclib::Point {
    match direction {
        aoclib::Direction::Left => (point.0 + 1, point.1),
        aoclib::Direction::Right => (point.0 - 1, point.1),
        aoclib::Direction::Up => (point.0, point.1 + 1),
        aoclib::Direction::Down => (point.0, point.1 - 1),
    }
}

fn tilt_grid(grid: &Grid, direction: &Direction) -> Grid {
    let mut new_grid = grid.clone();
    let mut points = new_grid.iterate_elements().collect_vec();
    if matches!(direction, Direction::Down | Direction::Right) {
        points.reverse();
    }
    for i in 0..points.len() {
        let (point, element) = points[i];
        match Rock::from_char(&element) {
            Rock::Square | Rock::Empty => continue,
            _ => (),
        }
        let new_point = match new_grid
            .iterate_from_point(point, direction.clone())
            .find(|(_, content)| !matches!(Rock::from_char(content), Rock::Empty))
        {
            Some((non_empty_point, _)) => point_before(non_empty_point, direction),
            None => point_edge(&new_grid, point, direction),
        };

        if new_point == point {
            continue;
        }
        // println!("Moved rock from {:?} to {:?}", point, new_point);
        // println!("Old Grid: \n{}", grid);
        new_grid.contents.insert(point, '.');
        new_grid.contents.insert(new_point, 'O');
        // println!("New Grid: \n{}", grid);
    }
    new_grid
}

// Find first index of something above current element
// If it's a rock, move it to Direction
fn solve_one(grid: &Grid) -> isize {
    let grid = tilt_grid(grid, &Direction::Up);
    grid.contents
        .iter()
        .fold(0, |acc, (k, c)| if !matches!(Rock::from_char(c), Rock::Round) { acc } else { acc + grid.height - k.1 })
}

// Find first index of something above current element
// If it's a rock, move it to Direction
fn solve_two(grid: &Grid, cycles: usize) -> isize {
    let cycle_directions = vec![Direction::Up, Direction::Left, Direction::Down, Direction::Right];
    let mut grid = (*grid).clone();
    let mut finished_cycles: Vec<Grid> = vec![];
    let mut i_unique = 0;
    let mut repeat_length = 0;
    // After some amount of cycles the pattern just infinitely repeats itself and we have cached all the
    // results
    for i in 0..cycles {
        // Tilt the grid four times.
        for direction in cycle_directions.iter() {
            grid = tilt_grid(&grid, direction);
        }
        // If we already have this result, the cycle starts repeating itself
        if let Some((res, _)) = finished_cycles.iter().find_position(|x| *x == &grid) {
            // Calculate the amount of items that repeat
            repeat_length = i - res;
            break;
        } else {
            finished_cycles.push(grid.clone());
        }
    }
    // Calculate the position in the array of cached results that maps to
    // the amount of cycles by doing some weird modulo-esque operation.
    // I bet this is dumb. But it works.
    let mut j = cycles;
    while j > finished_cycles.len() - 1 {
        j -= repeat_length;
    }
    // Had an off-by-one error. Fixed :)!
    j -= 1;
    finished_cycles[j]
        .contents
        .iter()
        .fold(0, |acc, (k, c)| if !matches!(Rock::from_char(c), Rock::Round) { acc } else { acc + grid.height - k.1 })
}

fn main() {
    let input = include_str!("../input.txt.real");
    let mut grid = Grid::from(input);
    let now = Instant::now();
    println!("Solution one: {}", solve_one(&grid));
    println!("Solution two: {}", solve_two(&grid, 1000000000));
    println!("Elapsed: {:?}", now.elapsed());
}
