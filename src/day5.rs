use crate::common;
use anyhow::Result;
use std::iter::once;
use std::ops::RangeInclusive;

pub fn main() -> Result<(i64, usize)> {
    let lines = common::read_lines("inputs/5.txt")?;
    let mut solution_a = 0;

    let mut fresh_ranges = Vec::new();
    let mut reading_ranges = true;

    for line in lines {
        let line = line?;
        let line = line.trim();

        if reading_ranges {
            if line.is_empty() {
                reading_ranges = false;
                continue;
            }
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse()?;
            let end = end.parse()?;

            add_range(&mut fresh_ranges, start, end);
        } else {
            let id: i64 = line.parse()?;
            if fresh_ranges.iter().any(|range| range.contains(&id)) {
                solution_a += 1;
            }
        }
    }
    let solution_b = fresh_ranges.into_iter().map(RangeInclusive::count).sum();

    Ok((solution_a, solution_b))
}

fn add_range(ranges: &mut Vec<RangeInclusive<i64>>, mut start: i64, mut end: i64) {
    for i in 0..ranges.len() {
        if *ranges[i].end() < start {
            continue;
        }

        for j in i..ranges.len() {
            if *ranges[j].start() <= end {
                start = start.min(*ranges[j].start());
                end = end.max(*ranges[j].end());
                ranges[i] = (*ranges[i].start())..=end.max(*ranges[j].end());
            } else {
                ranges.splice(i..j, once(start..=end));
                return;
            }
        }
        ranges.splice(i.., once(start..=end));
        return;
    }
    ranges.push(start..=end);
}
