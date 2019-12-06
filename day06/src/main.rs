use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() {
    println!("{}", run(get_inputs()));
}

fn run(orbits: Vec<(String, String)>) -> i32 {
    let orbit_map = orbits.into_iter().collect::<HashMap<_,_>>();
    
    part1_total_orbits(orbit_map)
}

fn part1_total_orbits(orbit_map: HashMap<String, String>) -> i32 {
    orbit_map
        .keys()
        .fold(0, |acc, orbit| {
            let number_of_orbited = {
                let mut count = 0i32;
                let mut _orbit = orbit;
                while orbit_map.contains_key(_orbit) {
                    count +=1;
                    _orbit = orbit_map.get(_orbit).expect("Expected to find orbit.")
                }
                count
            };
            acc + number_of_orbited
        })
}

fn get_inputs() -> Vec<(String, String)> {
    let file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/inputs/day06.txt").expect("Couldn´t open file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Expected a line"))
        .map(|line| line.split(")").map(str::to_owned).collect())
        .map(|x: Vec<String>| (x[1].clone(), x[0].clone()))
        .collect::<Vec<(String, String)>>()
}