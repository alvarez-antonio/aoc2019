use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

fn main() {
    println!("{}", run(get_inputs()));
}

fn run(orbits: Vec<(String, String)>) -> i32 {
    let orbit_map = orbits.into_iter().collect::<HashMap<_,_>>();
    
    part2_distance_san_you(&orbit_map)
}

fn part2_distance_san_you(orbit_map: &HashMap<String, String>) -> i32 {
    distance_between_orbits(orbit_map, orbit_map.get("YOU").expect("should exist YOU").to_owned(), orbit_map.get("SAN").expect("Should exist SAN").to_owned())
}

fn distance_between_orbits(orbit_map: &HashMap<String, String>, orbit_key1: String, orbit_key2: String) -> i32 {
    let orbits1 = orbit_list(&orbit_map, orbit_key1);
    let orbits2 = orbit_list(&orbit_map, orbit_key2);
    
    for (i, orbit1) in orbits1.iter().enumerate() {
        for (j, orbit2) in orbits2.iter().enumerate() {
            if orbit1 == orbit2 {
                return (i + j) as i32
            }
        }
    }

    1i32
}

fn orbit_list(orbit_map: &HashMap<String, String>, from_orbit: String) -> Vec<String> {
    let mut list = vec![from_orbit.clone()];
    let mut orbit = from_orbit.as_str();
    while orbit_map.contains_key(orbit) {
        orbit = orbit_map.get(orbit).expect("orbit should exist");
        list.push(orbit.to_owned());
    }
    list
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