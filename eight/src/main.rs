use std::{collections::HashMap, fs::read_to_string};

use aoclib::consume_line;
use nom::{
  bytes::complete::tag,
  character::complete::{alpha1, alphanumeric1, multispace1},
  error::context,
  multi::separated_list1,
  sequence::{delimited, terminated, tuple},
  IResult,
};
use num::Integer;

/// Direction is a tuple of two strings
/// the first is the left turn and the second is the right turn
type Direction<'a> = (&'a str, &'a str);

/// LLLRRRRLRLRLR\n\n
fn directions(input: &str) -> IResult<&str, &str> {
  terminated(alpha1, tuple((consume_line, consume_line)))(input)
}

/// DGK =
fn index(input: &str) -> IResult<&str, &str> {
  context("index", terminated(alphanumeric1, tag(" = ")))(input)
}

/// (DHL, RED)
fn left_right(input: &str) -> IResult<&str, (&str, &str)> {
  let (rem, (l, _, r)) = context(
    "left_right",
    delimited(
      tag("("),
      nom::sequence::tuple((alphanumeric1, tag(", "), alphanumeric1)),
      tag(")"),
    ),
  )(input)?;
  Ok((rem, (l, r)))
}

fn direction_map(input: &str) -> IResult<&str, HashMap<&str, Direction>> {
  let mut map: HashMap<&str, Direction> = HashMap::new();
  let (rem, lines) = context(
    "direction_map",
    separated_list1(multispace1, tuple((index, left_right))),
  )(input)?;
  for line in lines {
    map.insert(line.0, line.1);
  }
  Ok((rem, map))
}

fn solution_one(directions: &str, map: &HashMap<&str, Direction>) -> i32 {
  let mut mem = map.get("AAA").unwrap();
  let mut steps = 0;
  loop {
    for c in directions.chars() {
      steps += 1;
      match c {
        'L' => {
          if mem.0 == "ZZZ" {
            return steps;
          }
          mem = map.get(mem.0).unwrap();
        }
        'R' => {
          if mem.1 == "ZZZ" {
            return steps;
          }
          mem = map.get(mem.1).unwrap();
        }
        _ => {
          panic!("invalid direction");
        }
      }
    }
  }
}

/// My initial solution would've taken 12.07 days to complete
fn solution_two(directions: &str, map: &HashMap<&str, Direction>) -> u64 {
  let mut starting_points = map
    .iter()
    .filter(|(k, _)| k.ends_with('A'))
    .map(|(k, v)| v)
    .collect::<Vec<_>>();
  let mut steps_count: Vec<u64> = vec![];
  'outer: for sp in starting_points.iter_mut() {
    let mut steps = 0_u64;
    loop {
      for c in directions.chars() {
        steps += 1;
        match c {
          'L' => {
            if sp.0.ends_with('Z') {
              steps_count.push(steps);
              continue 'outer;
            }
            *sp = map.get(sp.0).unwrap();
          }
          'R' => {
            if sp.1.ends_with('Z') {
              steps_count.push(steps);
              continue 'outer;
            }
            *sp = map.get(sp.1).unwrap();
          }
          _ => {
            panic!("invalid direction");
          }
        }
      }
    }
  }
  steps_count.iter().fold(1, |acc, x| acc.lcm(x))
}

fn main() {
  let input = read_to_string("input.txt.real").unwrap();
  // let input = read_to_string("input.txt").unwrap();
  let (rem, directions) = directions(&input).unwrap();
  let (_, map) = match direction_map(rem) {
    Ok(m) => m,
    Err(e) => {
      println!("error: {:#?}", e);
      panic!("error");
    }
  };
  let steps = solution_one(directions, &map);
  println!("Solution one: {}", steps);
  let steps = solution_two(directions, &map);
  println!("Solution two: {}", steps);
}
