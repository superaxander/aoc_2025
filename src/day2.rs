use crate::common;
use anyhow::Result;

pub fn main() -> Result<(u64, u64)> {
    let mut lines = common::read_lines("inputs/2.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let line = lines.next().unwrap();
    let line = line?;
    let line = line.trim();

    for range in line.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        let start: u64 = start.parse()?;
        let end: u64 = end.parse()?;

        for i in start..=end {
            let digits = 1 + i.ilog10();
            let factor = 10u64.pow(digits / 2);
            if i / factor == i % factor {
                solution_a += i;
            }

            'length_loop: for l in 1..=digits / 2 {
                if digits % l != 0 {
                    continue;
                }

                let factor = 10u64.pow(l);
                let num = i % factor;
                let mut mut_i = i;

                while mut_i != 0 {
                    if mut_i % factor != num {
                        continue 'length_loop;
                    }
                    mut_i /= factor;
                }

                solution_b += i;
                break;
            }
        }
    }

    Ok((solution_a, solution_b))
}
