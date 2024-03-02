use std::{collections::HashMap, fs::read_to_string, iter::repeat, time::Instant};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Record {
    springs: String,
    groups: Vec<usize>,
}

fn solve_record(springs: &str, groups: &[usize], memo: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    let mut total = 0;

    // No more groups to fill, which means we might've reached the end and found a possible way
    if groups.is_empty() {
        // No more broken springs left, no more groups left. We succeeded!
        if !springs.contains("#") {
            return 1;
        }
        // Still broken springs left, but no groups... this isn't valid.
        return 0;
    }

    // We already know the answer, return it.
    if let Some(r) = memo.get(&(springs.to_string(), groups.to_vec())) {
        return *r;
    }

    // Based on the amount of springs needed to fill groups, we check if current is valid
    let minimum_remaining_length = groups.iter().sum::<usize>() + groups.len() - 1;
    if springs.len() < minimum_remaining_length {
        return 0;
    }

    // Skip spring if its period
    if &springs[0..1] == "." {
        return solve_record(&springs[1..], groups, memo);
    }

    let cur_group = groups[0];

    // Springs are valid if they don't contain a . and the group fits
    let all_springs_valid = springs[0..cur_group].chars().all(|c| c != '.');
    // If we reached the last bit of springs, or there is NOT a spring after our current group, we are still valid.
    let last_char_valid = springs.len() == cur_group || springs[cur_group..cur_group + 1].chars().all(|c| c != '#');
    // Our current situation is valid, so let's match a group and move ahead to see if we can
    // match the next groups as well
    if all_springs_valid && last_char_valid {
        // max_idx is either after the current group, or the end of the springs
        // This takes into account the space needed after a valid group.
        let max_idx = springs.len().min(cur_group + 1);
        total += solve_record(&springs[max_idx..], &groups[1..], memo);
    }

    if &springs[0..1] != "#" {
        total += solve_record(&springs[1..], &groups, memo);
    }

    // Save answer in memo
    memo.insert((springs.to_string(), groups.to_vec()), total);
    total
}

fn solve_records(records: Vec<Record>, memo: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    let mut total = 0;
    for record in records.iter() {
        total += solve_record(&record.springs, &record.groups, memo);
    }
    total
}

fn parse_input(input: &str) -> Vec<Record> {
    let mut records = vec![];
    for line in input.lines() {
        let (springs, groups) = line.split_once(" ").unwrap();
        let springs = springs.to_string();
        let groups = groups.split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec();
        records.push(Record { springs, groups });
    }
    records
}

fn parse_input_two(input: &str) -> Vec<Record> {
    let mut records = vec![];
    for line in input.lines() {
        let (springs, groups) = line.split_once(" ").unwrap();
        let springs = repeat(springs).take(5).collect::<Vec<_>>().join("?");
        let groups = groups.split(",").map(|x| x.parse::<usize>().unwrap()).collect_vec().repeat(5);
        records.push(Record { springs, groups });
    }
    records
}

fn main() {
    let now = Instant::now();
    let input = read_to_string("input.txt.real").unwrap();
    let records = parse_input(&input);
    let solution_one = solve_records(records, &mut HashMap::new());
    println!("solution_one: {:?}", solution_one);
    let records = parse_input_two(&input);
    let solution_two = solve_records(records, &mut HashMap::new());
    println!("solution_two: {:?}", solution_two);
    let elapsed = now.elapsed();
    println!("elapsed: {:?}", elapsed);
}
