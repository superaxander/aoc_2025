use crate::common;
use crate::common::{CharConvertable, InfiniteGrid, SignedCoordinate};
use anyhow::Result;
use rustc_hash::FxHashSet;

#[derive(Copy, Clone, PartialEq)]
enum Data {
    ToiletRoll,
}

impl CharConvertable for Data {
    fn to_char(option: Option<&Self>) -> char {
        match option {
            None => '.',
            Some(Data::ToiletRoll) => '@',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => None,
            '@' => Some(Data::ToiletRoll),
            _ => panic!("Unexpected char: {c}"),
        }
    }
}

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/4.txt")?;
    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut map = InfiniteGrid::<SignedCoordinate, Data, false>::read(lines.map_while(Result::ok));
    let mut to_remove = FxHashSet::default();

    loop {
        for (coord, _) in map.entries() {
            if coord
                .neighbours::<true>()
                .filter(|n| map.get(n).is_some() && !to_remove.contains(n))
                .count()
                < 4
            {
                to_remove.insert(coord);
            }
        }

        if to_remove.is_empty() {
            break;
        }

        if solution_a == 0 {
            solution_a = to_remove.len();
        }
        solution_b += to_remove.len();

        for c in to_remove.drain() {
            map.set(c, None);
        }
    }

    Ok((solution_a, solution_b))
}
