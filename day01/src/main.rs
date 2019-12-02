use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp::max;

fn main() {
    
    let total_mass: i32 = part2_mass();

    print!("{}", total_mass);
}

fn part1_mass() -> i32 {
    get_inputs()
            .iter()
            .map( |mass| calculate_fuel_for_mass(*mass))
            .sum()
}

fn part2_mass() -> i32 {
    get_inputs()
        .iter()
        .map( |mass| complete_fuel_compute(*mass))
        .sum()
}

fn calculate_fuel_for_mass(mass: i32) -> i32 {
    max(0,( mass / 3 ) - 2)
}

fn complete_fuel_compute(mass: i32) -> i32 {
    let fuel = calculate_fuel_for_mass(mass);
    if fuel == 0 {
        0
    } else {
        fuel + complete_fuel_compute(fuel)
    }
}

fn get_inputs() -> Vec<i32> {
    let file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/inputs/day01.txt").expect("Couldn´t open file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Expected a line").parse::<i32>().expect("Couldn´t parse to i32"))
        .collect::<Vec<i32>>()
}

#[test]
fn basic_test() {
    assert_eq!(calculate_fuel_for_mass(12), 2);
}

#[test]
fn rounding_test() {
    assert_eq!(calculate_fuel_for_mass(14), 2);
}

#[test]
fn large_test() {
    assert_eq!(calculate_fuel_for_mass(1969), 654);
}

#[test]
fn larger_test() {
    assert_eq!(calculate_fuel_for_mass(100756), 33583);
}

#[test]
fn part2_basic_test() {
    assert_eq!(complete_fuel_compute(12), 2);
}

#[test]
fn part2_rounding_test() {
    assert_eq!(complete_fuel_compute(14), 2);
}

#[test]
fn part2_large_test() {
    assert_eq!(complete_fuel_compute(1969), 966);
}

#[test]
fn part2_larger_test() {
    assert_eq!(complete_fuel_compute(100756), 50346);
}