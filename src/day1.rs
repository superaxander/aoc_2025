use crate::common;
use anyhow::Result;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/1.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut num = 50;

    for line in lines {
        let line = line?;
        let line = line.trim();

        let direction = if &line[..1] == "L" { -1 } else { 1 };

        let amount: i64 = line[1..].parse()?;

        for _ in 0..amount {
            num = (num + direction) % 100;
            if num == 0 {
                solution_b += 1;
            }
        }

        if num == 0 {
            solution_a += 1;
        }
    }

    Ok((solution_a, solution_b))
}
