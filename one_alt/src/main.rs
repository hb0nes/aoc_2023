use std::{fs::File, io::Read, vec};

const STRING_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn solve(lines: Vec<&str>) -> (i32, i32) {
    let mut sum = 0;
    let mut sum_two = 0;
    for line in lines {
        let mut digits_one: Vec<i32> = vec![];
        let mut digits_two: Vec<i32> = vec![];
        let mut start = 0;
        while let Some(c) = line[start..].chars().next() {
            match c {
                d if d.is_numeric() => {
                    digits_one.push(d.to_digit(10).unwrap() as i32);
                    digits_two.push(d.to_digit(10).unwrap() as i32);
                }
                _ => {
                    if let Some((i, _)) = STRING_DIGITS
                        .iter()
                        .enumerate()
                        .find(|(_, &s)| line[start..].starts_with(s))
                    {
                        digits_two.push((i + 1) as i32)
                    }
                }
            }
            start += 1;
        }
        sum += digits_one[0] * 10 + digits_one[digits_one.len() - 1];
        sum_two += digits_two[0] * 10 + digits_two[digits_two.len() - 1];
    }
    (sum, sum_two)
}

fn main() {
    let mut buf = String::new();
    File::read_to_string(&mut File::open("input.txt").unwrap(), &mut buf).unwrap();
    let lines: Vec<&str> = buf.split("\n").collect();
    let (ans_one, ans_two) = solve(lines);
    println!("Part one answer: {}", ans_one);
    println!("Part two answer: {}", ans_two);
}
