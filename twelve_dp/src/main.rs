use std::{collections::HashMap, fs::read_to_string, iter::repeat, time::Instant};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
struct Record {
    springs: String,
    groups: Vec<usize>,
}

fn solve_record_top(springs: &str, groups: &[usize], memo: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    let mut total = 0;

    // No more groups to fill, which means we might've reached the end and found a possible way
    if groups.is_empty() {
        // No more broken springs left, no more groups left. We succeeded!
        if !springs.contains('#') {
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
        return solve_record_top(&springs[1..], groups, memo);
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
        total += solve_record_top(&springs[max_idx..], &groups[1..], memo);
    }

    // We have checked only one possibility so far. Branch out to the next position to see if it's valid.
    // We cannot do this if our current position is a #, because it could result in something like this:
    // [##]?? -> #[#?]?
    // This would be valid from the perspective of this recursive function, as it doesn't look behind
    // so it would see #?.
    // However, groups need to be separated by ? or ., so it's actually not valid.
    if &springs[0..1] != "#" {
        total += solve_record_top(&springs[1..], groups, memo);
    }

    // Save answer in memo
    memo.insert((springs.to_string(), groups.to_vec()), total);
    total
}

fn solve_record_bottom(springs: &str, groups: &[usize]) -> usize {
    let mut dp: Vec<Vec<usize>> = vec![vec![0; groups.len()]; springs.len() + groups[groups.len() - 1] + 1];
    let mut min_j = 0;
    'i: for i in 0..springs.len() {
        // Manage memory
        if i > 0 {
            dp[i - 1].clear();
        }
        for j in 0..groups.len() {
            // If first group is at a broken spring, we skip it from now on
            // The first group decides all the valid starting positions and its placement can never
            // be past the first #.
            let cur_char = &springs[i..i + 1];
            if j < min_j {
                continue;
            }
            if cur_char == "#" && j == 0 {
                min_j = 1;
            }
            // Skip periods
            if cur_char == "." {
                continue 'i;
            }
            // If group can't be placed here according to previous logic, continue
            if j > 0 && dp[i][j - 1] == 0 {
                continue;
            }
            // If remaining groups don't fit in remaining springs, continue
            if groups[j..].iter().sum::<usize>() + groups[j..].len() - 1 > springs[i..].len() {
                continue;
            }
            // if we are at last group and there are springs remaining, group isn't valid
            let at_last_group = j == groups.len() - 1;
            if at_last_group && springs[i + groups[j]..].chars().any(|c| c == '#') {
                continue;
            }
            // Check if current group is valid
            let max_idx = springs.len().min(i + groups[j]);
            let end_reached = max_idx == springs.len();
            let subsequent_character = springs.get(max_idx..max_idx + 1).unwrap_or("");
            let group_valid = springs[i..i + groups[j]].chars().all(|x| x == '?' || x == '#') && (end_reached || subsequent_character != "#");
            if !group_valid {
                continue;
            }

            // If our current group is valid, we add the amount of ways we can reach the next
            // starting location, to all indices up to and including a broken spring.
            // If there are no broken springs, that means all remaining positions are valid for the
            // next group. During next iterations, we can check if the next group fits there.
            // If it does, we can do the same thing and add the amount of ways we could get to the starting index for the group after that,
            // and so forth.
            // --------------------------------------------------
            //             01234567
            // Scenario 1: ??.??.?? 1,1,1
            // --------------------------------------------------
            //
            //       dp[0]      dp[1]      dp[2]      dp[3]      dp[4]      dp[5]      dp[6]      dp[7]      dp[8]       dp[9]     ]
            //     [ [0, 0, 0], [0, 0, 0], [1, 0, 0], [2, 0, 0], [2, 0, 0], [2, 2, 0], [2, 4, 0], [2, 4, 0], [2, 4, 4],  [2, 4, 8] ]
            // --------------------------------------------------
            //             0123456
            // Scenario 2: ??.#.?? 1,1,1
            // --------------------------------------------------
            //
            //       dp[0]      dp[1]      dp[2]      dp[3]      dp[4]      dp[5]      dp[6]      dp[7]      dp[8]     ]
            //     [ [0, 0, 0], [0, 0, 0], [1, 0, 0], [2, 0, 0], [0, 0, 0], [0, 2, 0], [0, 2, 0], [0, 2, 2], [0, 2, 4] ]
            let next_start_idx = (springs.len()).min(i + groups[j] + 1);
            let next_broken_idx = match springs[next_start_idx..].find('#') {
                Some(n) => next_start_idx + n,
                None => dp.len() - 1,
            };
            for k in next_start_idx..=next_broken_idx {
                if j > 0 {
                    dp[k][j] += dp[i][j - 1];
                } else {
                    dp[k][j] += 1;
                }
            }
        }
    }
    dp[dp.len() - 1][dp[dp.len() - 1].len() - 1]
}

fn solve_records_top(records: &[Record], memo: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    let mut total = 0;
    for record in records.iter() {
        total += solve_record_top(&record.springs, &record.groups, memo);
    }
    total
}

fn solve_records_bottom(records: &[Record]) -> usize {
    let mut total = 0;
    records.par_iter().map(|record| solve_record_bottom(&record.springs, &record.groups)).sum::<usize>()
}

fn parse_input(input: &str) -> Vec<Record> {
    let mut records = vec![];
    for line in input.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = springs.to_string();
        let groups = groups.split(',').map(|x| x.parse::<usize>().unwrap()).collect_vec();
        records.push(Record { springs, groups });
    }
    records
}

fn parse_input_two(input: &str) -> Vec<Record> {
    let mut records = vec![];
    for line in input.lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        let springs = repeat(springs).take(5).collect::<Vec<_>>().join("?");
        let groups = groups.split(',').map(|x| x.parse::<usize>().unwrap()).collect_vec().repeat(5);
        records.push(Record { springs, groups });
    }
    records
}

fn main() {
    let input = read_to_string("input.txt.real").unwrap();
    let records = parse_input(&input);

    let now = Instant::now();
    let solution_one_bottom = solve_records_bottom(&records);
    let elapsed = now.elapsed();
    println!("Solution one, bottom-up: {:?}, took: {:?}", solution_one_bottom, elapsed);
    let now = Instant::now();
    let solution_one_top = solve_records_top(&records, &mut HashMap::new());
    let elapsed = now.elapsed();
    println!("Solution one, top-down: : {:?}, took: {:?}", solution_one_top, elapsed);

    let records = parse_input_two(&input);

    let now = Instant::now();
    let solution_two_bottom = solve_records_bottom(&records);
    let elapsed = now.elapsed();
    println!("Solution two, bottom-up: {:?}, took: {:?}", solution_two_bottom, elapsed);
    let now = Instant::now();
    let solution_two_top = solve_records_top(&records, &mut HashMap::new());
    let elapsed = now.elapsed();
    println!("Solution two, top-down:: {:?}, took: {:?}", solution_two_top, elapsed);
}
