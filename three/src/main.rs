use std::{
    collections::{HashMap, HashSet},
    vec,
};

use aoclib::*;

/// Point is (x, y)
type Point = (i32, i32);

struct Grid {
    contents: HashMap<Point, char>,
    parts: HashMap<Point, Part>,
    width: i32,
    height: i32,
}

impl Grid {
    // add_parts inserts parts to its internal HashMap 'parts' to keep track of which coordinate contains which part
    fn add_part_points(&mut self, part: Part) {
        for point in part.points.clone() {
            self.parts.insert(point, part.clone());
        }
    }

    fn gear_points(&self) -> Vec<Point> {
        self.contents
            .iter()
            .filter(|(_, &c)| c == '*')
            .map(|(&p, _)| p)
            .collect()
    }

    fn gear_ratios(&self) -> Vec<i32> {
        self.gear_points()
            .iter()
            .map(|gear_point| {
                self.surrounding_points(&gear_point)
                    .iter()
                    .filter_map(|sp| self.parts.get(sp))
                    .collect::<HashSet<&Part>>()
            })
            .filter(|p| p.len() == 2)
            .map(|p| p.iter().fold(1, |acc, p| acc * p.number))
            .collect::<Vec<i32>>()
    }

    fn surrounding_points(&self, point: &Point) -> Vec<Point> {
        (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (point.0 + dx, point.1 + dy)))
            .filter(|p| p.0 < self.width && p.0 >= 0 && p.1 < self.height && p.1 >= 0)
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Part {
    points: Vec<Point>,
    number: i32,
    unique_id: i32,
}

impl Part {
    fn is_real_part(&self, grid: &Grid) -> bool {
        self.points
            .iter()
            .flat_map(|p| grid.surrounding_points(p))
            .map(|sp| grid.contents.get(&sp).unwrap())
            .any(|contents| !contents.is_digit(10) && contents != &'.')
    }
}

fn build_grid_and_parts(lines: Vec<String>) -> (Grid, Vec<Part>) {
    let mut grid: Grid = Grid {
        contents: HashMap::new(),
        parts: HashMap::new(),
        width: lines[0].len() as i32,
        height: lines.len() as i32,
    };
    let mut parts: Vec<Part> = vec![];
    let mut unique_id = 0;
    for (y, line) in lines.iter().enumerate() {
        let mut number = 0;
        let mut points: Vec<Point> = vec![];
        for (x, c) in line.chars().into_iter().enumerate() {
            grid.contents.insert((x as i32, y as i32), c);
            match c.is_digit(10) {
                true => {
                    number = number * 10 + c.to_string().parse::<i32>().unwrap();
                    points.push((x as i32, y as i32));
                }
                _ if number > 0 => {
                    unique_id += 1;
                    let part = Part {
                        points: points.clone(),
                        number,
                        unique_id,
                    };
                    grid.add_part_points(part.clone());
                    parts.push(part);
                    number = 0;
                    points.clear();
                }
                _ => (),
            }
            if x == line.len() - 1 && number > 0 {
                unique_id += 1;
                let part = Part {
                    points: points.clone(),
                    number,
                    unique_id,
                };
                grid.add_part_points(part.clone());
                parts.push(part);
            }
        }
    }
    (grid, parts)
}

fn main() {
    let lines = read_file("input.txt");
    let (grid, parts) = build_grid_and_parts(lines);
    let real_parts: Vec<Part> = parts.iter().cloned().fold(vec![], |mut acc, x| {
        if x.is_real_part(&grid) {
            acc.push(x)
        }
        acc
    });
    let ans_one = real_parts.iter().fold(0, |acc, p| acc + p.number);
    let ans_two = grid.gear_ratios().iter().fold(0, |acc, r| acc + r);
    println!("One: {}", ans_one);
    println!("Two: {}", ans_two);
}
