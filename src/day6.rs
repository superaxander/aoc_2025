use crate::common;
use anyhow::Result;
use regex::Regex;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/6.txt")?;
    let re_ws = Regex::new("\\s+")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut columns = vec![];
    let mut grid: Vec<Vec<char>> = vec![];

    for line in lines {
        let line = line?;
        grid.push(line.chars().collect());
        let line = line.trim();

        for (i, p) in re_ws.split(line).enumerate() {
            if columns.len() <= i {
                columns.push(vec![p.parse::<i64>()?]);
            } else if p.chars().next().unwrap().is_ascii_digit() {
                columns[i].push(p.parse()?);
            } else {
                solution_a += match p {
                    "+" => columns[i].iter().copied().reduce(|l, r| l + r),
                    "*" => columns[i].iter().copied().reduce(|l, r| l * r),
                    _ => panic!("Unexpected operator: {p}"),
                }
                .unwrap();
            }
        }
    }

    let mut last_pos = 0;
    let mut last_op = '?';

    for (i, c) in grid.last().unwrap().iter().enumerate() {
        match c {
            '+' | '*' => {
                if i != 0 {
                    solution_b += accumulate_vertical(&grid, last_pos, last_op, i);
                }
                last_pos = i;
                last_op = *c;
            }
            ' ' => {}
            _ => panic!("Unexpected char {c}"),
        }
    }

    solution_b += accumulate_vertical(
        &grid,
        last_pos,
        last_op,
        grid.iter().map(Vec::len).max().unwrap() + 1,
    );

    Ok((solution_a, solution_b))
}

fn accumulate_vertical(grid: &[Vec<char>], last_pos: usize, last_op: char, i: usize) -> i64 {
    let mut acc = 0;
    for x in last_pos..i - 1 {
        let mut n = 0;
        for line in grid.iter().take(grid.len() - 1) {
            match line.get(x) {
                Some(c @ '0'..='9') => {
                    n = n * 10 + i64::from(c.to_digit(10).unwrap());
                }
                Some(' ') | None => {}
                Some(c) => panic!("Unexpected char: {c}"),
            }
        }

        match last_op {
            '+' => acc += n,
            '*' if x == last_pos => acc = n,
            '*' => acc *= n,
            _ => panic!("Unexpected op: {last_op}"),
        }
    }
    acc
}
