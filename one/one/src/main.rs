use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

// Expects 1-9 and one-nine
fn word_to_number(word: &str) -> Option<i32> {
    if let Ok(num) = word.parse::<i32>() {
        return Some(num);
    }
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

// Returns all overlapping matches
fn overlapping_matches(re: &Regex, line: &str) -> Vec<String> {
    let mut start = 0;
    let mut caps = Vec::new();
    while let Some(mat) = re.find(&line[start..]) {
        caps.push(mat.as_str().to_string());
        start += mat.start() + 1;
    }
    caps
}

// Expects at least one match and converts it to a number
fn captures_to_number(caps: Vec<String>) -> i32 {
    assert!(caps.len() > 0);
    let f = word_to_number(&caps[0]).unwrap();
    let l = word_to_number(&caps[caps.len() - 1]).unwrap();
    format!("{}{}", f, l).parse::<i32>().unwrap()
}

// Solve puzzle.
// Another potentially faster way is to use one regex to get all matches and drop the word-based ones for solution 1.
fn solve(lines: Lines<BufReader<File>>) -> (i32, i32) {
    let res = vec![
        Regex::new(r"1|2|3|4|5|6|7|8|9").unwrap(),
        Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine").unwrap(),
    ];
    let (mut sum_one, mut sum_two) = (0, 0);
    for line in lines {
        let line = line.unwrap();
        for (i, re) in res.iter().enumerate() {
            let caps = overlapping_matches(re, &line);
            let num = captures_to_number(caps);
            if i == 0 {
                sum_one += num;
                continue;
            }
            sum_two += num;
        }
    }
    (sum_one, sum_two)
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let lines = reader.lines();
    let (sum_one, sum_two) = solve(lines);
    println!("sum_one: {}", sum_one);
    println!("sum_two: {}", sum_two);
}
