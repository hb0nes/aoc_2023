use std::{collections::HashMap, thread::current};

use aoclib::*;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct CardLine {
    card_number: i32,
    numbers_winning: Vec<i32>,
    numbers_given: Vec<i32>,
}

fn parse_line(input: &str) -> IResult<&str, CardLine> {
    map(
        tuple((card_number, numbers_winning, numbers_given)),
        |(card_number, numbers_winning, numbers_given)| CardLine {
            card_number,
            numbers_winning,
            numbers_given,
        },
    )(input)
}

fn numbers_winning(input: &str) -> IResult<&str, Vec<i32>> {
    let middle = tuple((opt(multispace1), tag("|"), opt(multispace1)));
    terminated(separated_list1(multispace1, digit_to_i32), middle)(input)
}

fn numbers_given(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(multispace1, digit_to_i32)(input)
}

fn card_number(input: &str) -> IResult<&str, i32> {
    map_res(
        terminated(
            preceded(tuple((tag("Card"), multispace1)), digit1),
            tuple((tag(":"), multispace1)),
        ),
        |digit_str: &str| digit_str.parse::<i32>(),
    )(input)
}

fn solution_one(card_lines: &Vec<CardLine>) -> i32 {
    card_lines
        .iter()
        .map(|card_line| {
            card_line
                .numbers_given
                .iter()
                .filter(|given| card_line.numbers_winning.contains(given))
                .count()
        })
        .filter(|&score| score > 0)
        .fold(0, |acc, score| match score {
            1 => acc + 1,
            _ => acc + 2i32.pow(score as u32 - 1),
        })
}

fn solution_two(card_lines: &Vec<CardLine>) -> i32 {
    card_lines
        .iter()
        .map(|card_line| {
            (
                card_line.card_number,
                card_line
                    .numbers_given
                    .iter()
                    .filter(|given| card_line.numbers_winning.contains(given))
                    .count(),
            )
        })
        .fold(
            HashMap::new(),
            |mut card_counts, (card_number, numbers_matching)| {
                let current_card_count = card_counts.entry(card_number).or_insert(1).clone();
                (1..=numbers_matching).for_each(|i| {
                    card_counts
                        .entry(card_number + i as i32)
                        .and_modify(|i| *i += current_card_count)
                        .or_insert(1 + current_card_count);
                });
                card_counts
            },
        )
        .values()
        .sum()
}

fn main() {
    let lines = read_file("input.txt");
    let input = lines.join("\n");
    let (_, card_numbers) = parse_input_lines(&input, parse_line).unwrap();
    let solution_one = solution_one(&card_numbers);
    let solution_two = solution_two(&card_numbers);
    println!("Solution one: {:?}", solution_one);
    println!("Solution two: {:?}", solution_two);
}
