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
  Ok((
    input,
    format!("{}{}", sign.unwrap_or(""), number).parse().unwrap(),
  ))
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
  separated_list1(space1, potentially_negative_number)(input)
}

fn finish_pattern(input: &Vec<i32>) -> i32 {
  let mut sequence = 0;
  let mut pyramid: Vec<Vec<i32>> = vec![];
  'outer: loop {
    for i in 0..(input.len() - 1) {
      let d = (input[i] - input[i + 1]).abs();
      pyramid[sequence].push(d);
    }
    if pyramid[sequence].iter().all(|&x| x == 0) {
      break 'outer;
    }
    sequence += 1;
  }
  // Pyramid is built, work our way back
  println!("{:?}", pyramid);
  0
}

fn solution_one(input: Vec<Vec<i32>>) -> i32 {
  for line in input {}
  todo!()
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let (_, input) = parse_input_lines(&input, parse_line).unwrap();
  let a = finish_pattern(&input[0]);
  println!("Part 1: {}", a);
}
