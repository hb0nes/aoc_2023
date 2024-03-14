use std::time::Instant;

use itertools::Itertools;

fn find_reflections(row_blocks: &[Vec<String>], col_blocks: &[Vec<String>], smudges: isize) -> Vec<usize> {
    let mut total = vec![];
    for i in 0..row_blocks.len() {
        let a = find_reflection(&row_blocks[i], true, smudges);
        let b = find_reflection(&col_blocks[i], false, smudges);
        total.push(a + b);
    }
    total
}

// Find two subsequent lines that are similar enough according to 'smudges'
// Keep track of how many smudges we have accounted for.
// When those two lines are found, expand outwards in both directions,
// checking if either direction has a valid reflection.
fn find_reflection(block: &[String], rows: bool, smudges: isize) -> usize {
    let mut total = 0;
    'j: for j in 0..block.len() - 1 {
        let mut smudges_left = smudges;
        smudges_left -= block[j].chars().zip(block[j + 1].chars()).filter(|(cur, next)| cur != next).count() as isize;
        if smudges_left < 0 {
            continue;
        }
        for d in 1..=j {
            if j + 1 + d >= block.len() {
                break;
            }
            smudges_left -= block[j - d].chars().zip(block[j + 1 + d].chars()).filter(|(cur, next)| cur != next).count() as isize;
            if smudges_left < 0 {
                continue 'j;
            }
        }
        if smudges_left == 0 {
            total += if rows { (j + 1) * 100 } else { j + 1 };
        }
    }
    total
}

fn main() {
    env_logger::init();
    let row_blocks: Vec<Vec<String>> = (include_str!("../input.txt.real"))
        .split("\n\n")
        .map(|block| block.split('\n').filter(|r| r != &"").map(|r| r.to_string()).collect_vec())
        .collect_vec();
    let mut col_blocks: Vec<Vec<String>> = vec![];
    for i in 0..row_blocks.len() {
        col_blocks.push(vec![]);
        let row_len = row_blocks[i][0].len();
        for j in 0..row_len {
            let mut col = "".to_string();
            for k in 0..row_blocks[i].len() {
                col += &row_blocks[i][k][j..j + 1];
            }
            col_blocks[i].push(col);
        }
    }
    let now = Instant::now();
    let total = find_reflections(&row_blocks, &col_blocks, 0).iter().sum::<usize>();
    println!("Solution 1: {:?}", total);
    let total = find_reflections(&row_blocks, &col_blocks, 1).iter().sum::<usize>();
    println!("Solution 2: {:?}", total);
    println!("elapsed: {:?}", now.elapsed());
}
