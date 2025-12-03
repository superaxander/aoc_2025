use crate::common;
use anyhow::Result;

pub fn main() -> Result<(u64, u64)> {
    let lines = common::read_lines("inputs/3.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();
        let chars: Vec<u64> = line
            .chars()
            .map(|c| u64::from(c.to_digit(10).unwrap()))
            .collect();

        let (max_pos, first_digit) = chars[..chars.len() - 1]
            .iter()
            .enumerate()
            .max_by(|x, y| Ord::cmp(&x.1, &y.1))
            .unwrap();
        let add = first_digit * 10 + chars[max_pos + 1..].iter().max().unwrap();
        solution_a += add;

        let (mut pos, mut num) = chars[..chars.len() - 11]
            .iter()
            .copied()
            .enumerate()
            .rev()
            .max_by(|x, y| Ord::cmp(&x.1, &y.1))
            .unwrap();
        for digit in (0..11).rev() {
            num *= 10;
            let (offset, add) = chars[pos + 1..chars.len() - digit]
                .iter()
                .enumerate()
                .rev()
                .max_by(|x, y| Ord::cmp(&x.1, &y.1))
                .unwrap();
            pos += offset + 1;
            num += add;
        }

        solution_b += num;
    }

    Ok((solution_a, solution_b))
}
