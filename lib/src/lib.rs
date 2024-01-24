use std::{
    fs::File,
    io::{BufReader, Read},
};

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
    BufReader::new(File::open(name).unwrap())
        .read_to_string(&mut buf)
        .unwrap();
    buf.split('\n').map(|s| s.to_string()).collect()
}
