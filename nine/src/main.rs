use std::fs::read_to_string;

use aoclib::parse_input_lines;
use nom::{
  bytes::complete::tag,
  character::complete::{digit1, space1},
  combinator::opt,
  multi::separated_list1,
  sequence::tuple,
  IResult,
};

fn potentially_negative_number(input: &str) -> IResult<&str, i32> {
  let (input, (sign, number)) = tuple((opt(tag("-")), digit1))(input)?;
  Ok((input, format!("{}{}", sign.unwrap_or(""), number).parse().unwrap()))
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
  separated_list1(space1, potentially_negative_number)(input)
}

/// finish_pattern builds a pyramid of diffs as per AoC instructions
/// Once we reach a sequence of 0's, we can finish the pattern by summing every
/// last element.
fn finish_pattern(input: Vec<i32>, forward: bool) -> i32 {
  let mut pyramid: Vec<Vec<i32>> = vec![input];
  // Build the pyramid until we reach a sequence of only 0's
  while !pyramid.last().unwrap().iter().all(|&x| x == 0) {
    pyramid.push(pyramid.last().unwrap().windows(2).map(|x| x[1] - x[0]).collect());
  }
  // Finish the pattern forwards, or backwards
  if forward {
    pyramid.iter().fold(0, |acc, seq| acc + *seq.last().unwrap())
  } else {
    pyramid.iter().rev().fold(0, |acc, seq| seq.first().unwrap() - acc)
  }
}

fn solve(input: &[Vec<i32>], part_one: bool) -> i32 {
  input.iter().fold(0, |acc, x| acc + finish_pattern(x.clone(), part_one))
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let (_, input_parsed) = parse_input_lines(&input, parse_line).unwrap();
  let solution_one = solve(&input_parsed, true);
  println!("Part 1: {}", solution_one);
  let solution_two = solve(&input_parsed, false);
  println!("Part 2: {}", solution_two);
}
