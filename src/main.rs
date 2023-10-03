

use std::env;
use std::fs;
use std::path::PathBuf;
#[allow(dead_code)]
mod day1;
#[allow(dead_code)]
mod day2;
#[allow(dead_code)]
mod day3;
#[allow(dead_code)]
mod day4;
#[allow(dead_code)]
mod day5;
#[allow(dead_code)]
mod day6;
#[allow(dead_code)]
mod day7;
#[allow(dead_code)]
mod day8;
#[allow(dead_code)]
mod day9;
#[allow(dead_code)]
mod day10;
#[allow(dead_code)]
mod day11;
#[allow(dead_code)]
mod day12;
#[allow(dead_code)]
mod day13;
#[allow(dead_code)]
mod day14;
#[allow(dead_code)]
mod day15;
#[allow(dead_code)]
mod day16;

extern crate aoc_lib;

fn main() {
    let year = "2022";
    let day = "15";
    let input = aoc_lib::input_reader::get_input(year, day, "cookie.txt");
    //day1::run(input);
    //day2::run(input);
    //day3::run(input);
    //day4::run(input);
    //day5::run(input);
    //day6::run(input);
    //day7::run(input);
    //day8::run(input);
    //day9::run(input);
    //day10::run(input);
    //day11::run(input);
    //day12::run(input);
    //day13::run(input);
    //day14::run(input);
    day15::run(input);
    //day16::run(input);
}