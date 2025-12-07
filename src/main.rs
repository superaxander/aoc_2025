#![feature(array_windows)]
#![feature(int_roundings)]
#![feature(iter_map_windows)]
#![feature(pattern)]
#![feature(iter_array_chunks)]
#![feature(gen_blocks)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use std::time::Instant;

use common::{Day, Runnable};
use tracing::{Level, info, info_span};

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
// mod day8;
// mod day9;

// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;

// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(Level::INFO)
        .init();
    let span = info_span!("All days");
    span.in_scope(|| {
        let start = Instant::now();
        Day::Combined(day1::main).run("day 1");
        Day::Combined(day2::main).run("day 2");
        Day::Combined(day3::main).run("day 3");
        Day::Combined(day4::main).run("day 4");
        Day::Combined(day5::main).run("day 5");
        Day::Combined(day6::main).run("day 6");
        Day::Combined(day7::main).run("day 7");
        // Day::Combined(day8::main).run("day 8");
        // Day::Combined(day9::main).run("day 9");
        // Day::Combined(day10::main).run("day 10");
        // Day::Combined(day11::main).run("day 11");
        // Day::Combined(day12::main).run("day 12");
        // Day::Combined(day13::main).run("day 13");
        // Day::Combined(day14::main).run("day 14");
        // Day::Combined(day15::main).run("day 15");
        // Day::Combined(day16::main).run("day 16");
        // Day::Combined(day17::main).run("day 17");
        // Day::Combined(day18::main).run("day 18");
        // Day::Combined(day19::main).run("day 19");
        // Day::Combined(day20::main).run("day 20");
        // Day::Combined(day21::main).run("day 21");
        // Day::Combined(day22::main).run("day 22");
        // Day::Combined(day23::main).run("day 23");
        // Day::Combined(day24::main).run("day 24");
        // Day::Combined(day25::main).run("day 25");

        info!("Took {:#?}", start.elapsed());
    });
}
