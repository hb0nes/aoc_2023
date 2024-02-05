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

/// finish_pattern builds a pyramid of diffs as per AoC instructions
/// Once we reach a sequence of 0's, we can finish the pattern by summing every
/// last element.
fn finish_pattern(input: Vec<i32>, forward: bool) -> i32 {
  let mut pyramid: Vec<Vec<i32>> = vec![input];
  loop {
    pyramid.push(vec![]);
    let idx_next = pyramid.len() - 1;
    let idx_cur = idx_next - 1;
    for i in 0..(pyramid[idx_cur].len() - 1) {
      let diff = pyramid[idx_cur][i + 1] - pyramid[idx_cur][i];
      pyramid[idx_next].push(diff);
    }
    // If we have a sequence of only 0's, we can break because we have all the info required
    // to finish the pattern.
    if pyramid[idx_next].iter().all(|&x| x == 0) {
      break;
    }
  }
  if forward {
    pyramid.iter().fold(0, |acc, x| acc + x[x.len() - 1])
  } else {
    pyramid.iter().rev().fold(0, |acc, x| x[0] - acc)
  }
}

fn solution_one(input: Vec<Vec<i32>>) -> i32 {
  input.iter().fold(0, |acc, x| {
    let a = finish_pattern(x.clone(), true);
    println!("acc: {:?}, fin: {:?}, x: {:?}", acc, a, x);
    acc + a
  })
}

fn solution_two(input: Vec<Vec<i32>>) -> i32 {
  input.iter().fold(0, |acc, x| {
    let a = finish_pattern(x.clone(), false);
    println!("acc: {:?}, fin: {:?}, x: {:?}", acc, a, x);
    acc + a
  })
}

fn main() {
  let input = read_to_string("input.txt").unwrap();
  let (_, input_parsed) = parse_input_lines(&input, parse_line).unwrap();
  let a = solution_one(input_parsed.clone());
  println!("Part 1: {}", a);
  let b = solution_two(input_parsed.clone());
  println!("Part 2: {}", b);
}
