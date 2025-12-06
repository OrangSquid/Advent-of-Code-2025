use std::{cmp::max, cmp::min, ops::RangeInclusive, u64};

use aoc_2025::readlines::read_lines;

fn coallesce(ranges: &mut Vec<RangeInclusive<u64>>, begin: u64, end: u64) {
    match ranges
        .iter()
        .enumerate()
        .find(|(_, range)| {
            range.contains(&begin) || range.contains(&end) ||
            (*range.start() > begin && *range.start() < end)
        })
    {
        Some((idx, range)) => {
            let new_begin = min(begin, *range.start());
            let new_end = max(end, *range.end());
            ranges.swap_remove(idx);
            coallesce(ranges, new_begin, new_end);
        }
        None => ranges.push(begin..=end),
    };
}

fn main() {
    let mut lines = read_lines("input/day5.txt");
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();

    while let Some(line) = lines.next()
        && line.len() > 0
    {
        let (left, right) = line.split_once("-").unwrap();
        let left_parse: u64 = left.parse().unwrap();
        let right_parse: u64 = right.parse().unwrap();
        coallesce(&mut ranges, left_parse, right_parse);
    }

    let mut result_1 = 0;

    while let Some(line) = lines.next() {
        let parsed_int: u64 = line.parse().unwrap();
        if ranges.iter().any(|x| x.contains(&parsed_int)) {
            result_1 += 1;
        }
    }

    let result_2: u64 = ranges.iter().map(|x| x.end() - x.start() + 1).sum();

    println!("{ranges:?}");

    println!("{result_1}");
    println!("{result_2}");
}
