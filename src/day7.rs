use crate::common;
use crate::common::{CharConvertable, InfiniteGrid, SignedCoordinate};
use anyhow::Result;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Data {
    Start,
    Splitter,
}

impl CharConvertable for Data {
    fn to_char(option: Option<&Self>) -> char {
        match option {
            None => '.',
            Some(Data::Start) => 'S',
            Some(Data::Splitter) => '^',
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => None,
            'S' => Some(Data::Start),
            '^' => Some(Data::Splitter),
            _ => panic!("Unexpected char: {c}"),
        }
    }
}

pub fn main() -> Result<(i64, usize)> {
    let lines = common::read_lines("inputs/7.txt")?;

    let mut solution_a = 0;

    let grid = InfiniteGrid::<SignedCoordinate, Data, false>::read(lines.map_while(Result::ok));
    let max = grid.extents().1;

    let start = grid.entries().find(|(_, d)| *d == Data::Start).unwrap().0;
    let mut beams = FxHashSet::from_iter([start]);
    let mut new_beams = FxHashSet::default();

    for _ in 0..max.y {
        for beam in beams.drain() {
            let down = beam.south(1);

            if grid.get(&down).is_some() {
                solution_a += 1;
                new_beams.insert(down.west(1));
                new_beams.insert(down.east(1));
            } else {
                new_beams.insert(down);
            }
        }
        std::mem::swap(&mut beams, &mut new_beams);
    }

    let solution_b = count_universes(
        &grid,
        &mut vec![FxHashMap::default(); max.y as usize],
        start,
    );

    Ok((solution_a, solution_b))
}

fn count_universes(
    grid: &InfiniteGrid<SignedCoordinate, Data, false>,
    caches: &mut [FxHashMap<SignedCoordinate, usize>],
    beam: SignedCoordinate,
) -> usize {
    let down = beam.south(1);

    if let Some((cache, rest)) = caches.split_first_mut() {
        macro_rules! get_or_insert {
            ($pos:expr) => {
                *cache
                    .entry($pos)
                    .or_insert_with(|| count_universes(grid, rest, $pos))
            };
        }
        if grid.get(&down).is_some() {
            get_or_insert!(down.west(1)) + get_or_insert!(down.east(1))
        } else {
            get_or_insert!(down)
        }
    } else if grid.get(&down).is_some() {
        2
    } else {
        1
    }
}
