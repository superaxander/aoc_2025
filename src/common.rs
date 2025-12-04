#![allow(dead_code)]

use core::convert::AsRef;
use core::result::Result::Ok;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Div, Mul, Sub};
use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use rustc_hash::FxHashMap;
use tracing::{error, info};

pub fn read_lines<P>(filename: P) -> Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[allow(dead_code)]
pub enum Day<SolutionA: Display, SolutionB: Display> {
    Combined(fn() -> Result<(SolutionA, SolutionB)>),
    Separated(fn() -> Result<SolutionA>, fn() -> Result<SolutionA>),
    BoolSeparated(fn(bool) -> Result<SolutionA>),
}

impl<SolutionA: Display, SolutionB: Display> Day<SolutionA, SolutionB> {
    fn run_with_result(&self, name: &str) -> Result<()> {
        match self {
            Day::Combined(func) => {
                let now = Instant::now();
                let (solution_a, solution_b) = func()?;
                info!("Combined parts took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, solution_a);
                info!("Solution {}b: {}", name, solution_b);
            }
            Day::Separated(func_a, func_b) => {
                let now = Instant::now();
                let solution_a = func_a()?;
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let solution_b = func_b()?;
                info!("Part b took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, solution_a);
                info!("Solution {}b: {}", name, solution_b);
            }
            Day::BoolSeparated(func) => {
                let now = Instant::now();
                let solution_a = func(false)?;
                info!("Part a took {:#?}", now.elapsed());
                let now = Instant::now();
                let solution_b = func(true)?;
                info!("Part b took {:#?}", now.elapsed());
                info!("Solution {}a: {}", name, solution_a);
                info!("Solution {}b: {}", name, solution_b);
            }
        }
        Ok(())
    }
}

pub(crate) trait Runnable {
    fn run(&self, name: &str);
}

impl<SolutionA: Display, SolutionB: Display> Runnable for Day<SolutionA, SolutionB> {
    fn run(&self, name: &str) {
        if let Err(e) = self.run_with_result(name) {
            error!("Error occurred running {}: {}", name, e);
        }
    }
}

pub trait Coordinate: Clone + Eq + PartialEq + Hash + From<(usize, usize)> {
    fn min(&self, other: &Self) -> Self;

    fn max(&self, other: &Self) -> Self;

    fn range_to_debug(&self, other: &Self) -> impl Iterator<Item = (bool, Self)>;
}

pub trait CharConvertable: Sized {
    fn to_char(option: Option<&Self>) -> char;
    fn from_char(c: char) -> Option<Self>;
}

impl CharConvertable for char {
    fn to_char(option: Option<&Self>) -> char {
        match option {
            None => '.',
            Some(c) => *c,
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => None,
            c => Some(c),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct InfiniteGrid<
    Coord: Coordinate,
    Data: PartialEq,
    const CACHED_EXTENTS: bool,
    const INCLUDE_EMPTY: bool = false,
> {
    map: FxHashMap<Coord, Data>,
    min: Option<Coord>,
    max: Option<Coord>,
}

impl<
    Coord: Coordinate,
    Data: Clone + CharConvertable + PartialEq,
    const CACHED_EXTENTS: bool,
    const INCLUDE_EMPTY: bool,
> InfiniteGrid<Coord, Data, CACHED_EXTENTS, INCLUDE_EMPTY>
{
    pub fn read(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        // Include empty makes no sense if we aren't caching
        debug_assert!(CACHED_EXTENTS || !INCLUDE_EMPTY);
        let mut grid = Self {
            map: FxHashMap::default(),
            min: None,
            max: None,
        };
        for (y, line) in lines.enumerate() {
            for (x, c) in line.as_ref().trim().chars().enumerate() {
                let coord = Coord::from((x, y));
                let result = grid.set(coord, Data::from_char(c));
                debug_assert!(result.is_none());
            }
        }
        grid
    }

    pub fn get(&self, coord: &Coord) -> Option<&Data> {
        self.map.get(coord)
    }

    pub fn set(&mut self, coord: Coord, data: Option<Data>) -> Option<Data> {
        match data {
            None => {
                if CACHED_EXTENTS && INCLUDE_EMPTY {
                    self.min = Some(
                        self.min
                            .take()
                            .map_or_else(|| coord.clone(), |m| m.min(&coord)),
                    );
                    self.max = Some(
                        self.max
                            .take()
                            .map_or_else(|| coord.clone(), |m| m.max(&coord)),
                    );
                }
                self.map.remove(&coord)
            }
            Some(data) => {
                if CACHED_EXTENTS {
                    self.min = Some(
                        self.min
                            .take()
                            .map_or_else(|| coord.clone(), |m| m.min(&coord)),
                    );
                    self.max = Some(
                        self.max
                            .take()
                            .map_or_else(|| coord.clone(), |m| m.max(&coord)),
                    );
                }
                self.map.insert(coord, data)
            }
        }
    }

    pub fn extents(&self) -> (Coord, Coord) {
        if CACHED_EXTENTS {
            (
                self.min.as_ref().unwrap().clone(),
                self.max.as_ref().unwrap().clone(),
            )
        } else {
            (
                self.map.keys().cloned().reduce(|a, b| a.min(&b)).unwrap(),
                self.map.keys().cloned().reduce(|a, b| a.max(&b)).unwrap(),
            )
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = (Coord, Data)> + '_ {
        self.map
            .iter()
            .map(|(coord, data)| (coord.clone(), data.clone()))
    }
}

impl<
    Coord: Coordinate + Debug,
    Data: Clone + CharConvertable + PartialEq,
    const CACHED_EXTENTS: bool,
    const INCLUDE_EMPTY: bool,
> Debug for InfiniteGrid<Coord, Data, CACHED_EXTENTS, INCLUDE_EMPTY>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.extents();
        for (nl, coord) in min.range_to_debug(&max) {
            if nl {
                writeln!(f, "{}", Data::to_char(self.get(&coord)))?;
            } else {
                write!(f, "{}", Data::to_char(self.get(&coord)))?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SignedCoordinate {
    pub x: i64,
    pub y: i64,
}

impl From<(usize, usize)> for SignedCoordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl SignedCoordinate {
    pub const ZERO: SignedCoordinate = SignedCoordinate { x: 0, y: 0 };

    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn north(&self, amount: i64) -> SignedCoordinate {
        SignedCoordinate {
            x: self.x,
            y: self.y - amount,
        }
    }
    pub fn east(&self, amount: i64) -> SignedCoordinate {
        SignedCoordinate {
            x: self.x + amount,
            y: self.y,
        }
    }
    pub fn south(&self, amount: i64) -> SignedCoordinate {
        SignedCoordinate {
            x: self.x,
            y: self.y + amount,
        }
    }
    pub fn west(&self, amount: i64) -> SignedCoordinate {
        SignedCoordinate {
            x: self.x - amount,
            y: self.y,
        }
    }

    pub fn forward(&self, facing: Facing, amount: i64) -> SignedCoordinate {
        match facing {
            Facing::North => self.north(amount),
            Facing::East => self.east(amount),
            Facing::South => self.south(amount),
            Facing::West => self.west(amount),
        }
    }

    pub gen fn neighbours<const INCLUDE_DIAGONAL: bool>(&self) -> SignedCoordinate {
        if INCLUDE_DIAGONAL {
            for y in -1..=1 {
                for x in -1..=1 {
                    if x != 0 || y != 0 {
                        yield SignedCoordinate::new(self.x + x, self.y + y);
                    }
                }
            }
        } else {
            yield self.north(1);
            yield self.west(1);
            yield self.east(1);
            yield self.south(1);
        }
    }
}

impl Debug for SignedCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for SignedCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coordinate for SignedCoordinate {
    fn min(&self, other: &Self) -> Self {
        SignedCoordinate {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(&self, other: &Self) -> Self {
        SignedCoordinate {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    fn range_to_debug(&self, other: &Self) -> impl Iterator<Item = (bool, Self)> {
        (self.y..=other.y).flat_map(move |y| {
            (self.x..=other.x).map(move |x| {
                if x == other.x {
                    (true, SignedCoordinate { x, y })
                } else {
                    (false, SignedCoordinate { x, y })
                }
            })
        })
    }
}

impl Add for SignedCoordinate {
    type Output = SignedCoordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for SignedCoordinate {
    type Output = SignedCoordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for SignedCoordinate {
    type Output = SignedCoordinate;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<SignedCoordinate> for i64 {
    type Output = SignedCoordinate;

    fn mul(self, rhs: SignedCoordinate) -> Self::Output {
        SignedCoordinate {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<i64> for SignedCoordinate {
    type Output = SignedCoordinate;

    fn div(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    pub fn left(self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
        }
    }

    pub fn right(self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Facing::North => Facing::South,
            Facing::East => Facing::West,
            Facing::South => Facing::North,
            Facing::West => Facing::East,
        }
    }
}

pub fn is_prime(i: i64) -> bool {
    for j in 2..i / 2 {
        if i % j == 0 {
            return false;
        }
    }
    true
}

pub fn url_encode(string: &str) -> String {
    let mut encoded = String::new();
    for c in string.chars() {
        match c {
            '\n' => encoded.push_str("%0A"),
            '\t' => encoded.push_str("%09"),
            ' ' => encoded.push_str("%20"),
            '!' => encoded.push_str("%21"),
            '"' => encoded.push_str("%22"),
            '#' => encoded.push_str("%23"),
            '$' => encoded.push_str("%24"),
            '%' => encoded.push_str("%25"),
            '&' => encoded.push_str("%26"),
            '\'' => encoded.push_str("%27"),
            '(' => encoded.push_str("%28"),
            ')' => encoded.push_str("%29"),
            '*' => encoded.push_str("%2A"),
            '+' => encoded.push_str("%2B"),
            ',' => encoded.push_str("%2C"),
            '/' => encoded.push_str("%2F"),
            ':' => encoded.push_str("%3A"),
            ';' => encoded.push_str("%3B"),
            '<' => encoded.push_str("%3C"),
            '>' => encoded.push_str("%3E"),
            '=' => encoded.push_str("%3D"),
            '?' => encoded.push_str("%3F"),
            '@' => encoded.push_str("%40"),
            '[' => encoded.push_str("%5B"),
            ']' => encoded.push_str("%5D"),
            '{' => encoded.push_str("%7B"),
            '}' => encoded.push_str("%7D"),
            _ => encoded.push(c),
        }
    }
    encoded
}
