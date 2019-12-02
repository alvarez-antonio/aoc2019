use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    
    let total_mass: i32 = get_inputs()
        .iter()
        .map( |mass| calculate_fuel(*mass))
        .sum();

    print!("{}", total_mass);
}

fn calculate_fuel(mass: i32) -> i32 {
    ( mass / 3 ) - 2
}

fn get_inputs() -> Vec<i32> {
    let file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/day01/src/input/input.txt").expect("Couldn´t open file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Expected a line").parse::<i32>().expect("Couldn´t parse to i32"))
        .collect::<Vec<i32>>()
}

#[test]
fn basic_test() {
    assert_eq!(calculate_fuel(12), 2);
}

#[test]
fn rounding_test() {
    assert_eq!(calculate_fuel(14), 2);
}

#[test]
fn large_test() {
    assert_eq!(calculate_fuel(1969), 654);
}

#[test]
fn larger_test() {
    assert_eq!(calculate_fuel(100756), 33583);
}