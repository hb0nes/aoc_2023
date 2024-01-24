use std::{
    fs::File,
    io::Read,
    ops::{Add, Sub},
};

use aoclib::consume_line;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

type ConversionBlock = Vec<ConversionEntry>;
type ConversionTable = Vec<ConversionBlock>;
#[derive(Debug, Clone)]
struct ConversionEntry {
    conversion_factor: i64,
    start: i64,
    end: i64,
}

fn seeds(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(
        tag("seeds: "),
        separated_list1(multispace1, map_res(digit1, |d: &str| d.parse())),
        multispace1,
    )(input)
}

fn conversion_block_start(input: &str) -> IResult<&str, ()> {
    consume_line(input)
}

fn create_conversion_entry(c: &[i64]) -> ConversionEntry {
    ConversionEntry {
        conversion_factor: c[0].sub(c[1]),
        start: c[1],
        end: c[1].add(c[2]).sub(1),
    }
}

fn conversion_entries(input: &str) -> IResult<&str, ConversionBlock> {
    map(
        preceded(
            conversion_block_start,
            separated_list1(multispace1, map_res(digit1, |d: &str| d.parse::<i64>())),
        ),
        |numbers| numbers.chunks(3).map(create_conversion_entry).collect(),
    )(input)
}

fn conversion_table(input: &str) -> IResult<&str, ConversionTable> {
    separated_list1(multispace1, conversion_entries)(input)
}

fn solution_one(input: &str) -> i64 {
    let (_, (seeds, conversion_table)) = tuple((seeds, conversion_table))(input).unwrap();
    seeds.iter().enumerate().fold(i64::MAX, |acc, (i, &seed)| {
        acc.min(conversion_table.iter().fold(seed, |acc, conversion_block| {
            conversion_block
                .iter()
                .find(|entry| acc >= entry.start && acc <= entry.end)
                .map(|maybe_found| acc.add(maybe_found.conversion_factor))
                .unwrap_or(acc)
        }))
    })
}

fn solution_two(input: &str) -> i64 {
    let (_, (seeds, conversion_table)) = tuple((seeds, conversion_table))(input).unwrap();
    print!("{:#?}", conversion_table);
    let mut ranges = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect::<Vec<_>>();
    for conversion_entries in &conversion_table {
        let mut i = 0;
        while i < ranges.len() {
            let mut start = ranges[i].0;
            let mut end = ranges[i].1;
            for c in conversion_entries {
                // Doesn't fall into range at all
                if start > c.end || end < c.start {
                    continue;
                }
                // Range has overlap before
                if start < c.start {
                    ranges.push((start, c.start - 1));
                    start = c.start;
                }
                // Range has overlap after
                if end > c.end {
                    ranges.push((c.end + 1, end));
                    end = c.end;
                }
                // Range must be fully contained within
                ranges[i] = (start + c.conversion_factor, end + c.conversion_factor);
            }
            i += 1;
        }
    }
    ranges.iter().map(|(a, _)| a).min().cloned().unwrap()
}

fn main() {
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let solution_one = solution_one(&input);
    let solution_two = solution_two(&input);
    println!("Solution one: {}", solution_one);
    println!("Solution two: {}", solution_two);
}
