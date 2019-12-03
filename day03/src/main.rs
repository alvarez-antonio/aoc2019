use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::fmt;

enum Direction { U, R, D, L }

#[derive(Debug)]
struct Point { x: i32, y: i32 }
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Copy for Point {} 
impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Movement { direction: Direction, distance: i32 }

type Path = Vec<Point>;

fn main() {
    print!("{}", run(get_inputs()))
}

fn run(path_strings: Vec<String>) -> i32 {
    let paths: Vec<Path> = path_strings
        .iter()
        .map(|path| parse_path(path.to_string()))
        .collect();

    let mut common: Vec<Point> = Vec::new();
    for p1 in &paths[0] {
        for p2 in &paths[1] {
            if p1 == p2 && p1 != &(Point {x: 0, y: 0}) {
                common.push(Point { x: p1.x, y: p1.y })
            }
        }
    }
    
    print!("{:?}\n", common);
    common
        .into_iter()
        .map(|point| compute_manhattan_distance_to_origin(point))
        .min()
        .expect("Minimum distance not found")
}

fn compute_manhattan_distance_to_origin(point: Point) -> i32 {
    (point.x).abs() + (point.y).abs()
}

fn get_inputs() -> Vec<String> {
    let file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/inputs/day03.txt").expect("Couldn´t open file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Expected a line").to_string())
        .collect::<Vec<String>>()
}

fn parse_path(input_string: String) -> Vec<Point> {
    let movements = input_string
        .split(",")
        .map(|piece| path_piece_to_movement(piece.to_string()));
    
    let mut full_path = vec![Point { x: 0, y: 0}];
    for movement in movements {
        full_path.append(&mut to_points(full_path.last().expect("couldn't find point"), movement))
    }
    full_path
}

fn path_piece_to_movement(piece: String) -> Movement {
    let (head, tail) = piece.split_at(1);
    Movement { 
        direction: to_direction(head.parse::<char>().expect("couldn't parse to char")), 
        distance: tail.parse::<i32>().expect("Couldn´t parse to i32") 
    }
}

fn to_direction(dir: char) -> Direction {
    match dir {
        'U' => Direction::U,
        'R' => Direction::R,
        'D' => Direction::D,
        'L' => Direction::L,
        _ => panic!("Unknown direction.")
    }
}

fn to_points(starting_point: &Point, movement: Movement) -> Vec<Point> {
    //print!("{} + ", starting_point);
    //print!("{}\n", movement.distance);
    let points = (1..=movement.distance)
        .map(|step| move_point(starting_point, &movement.direction, step))
        .collect();

    //print!("{:?}\n", points);
    points
}

fn move_point(starting_point: &Point, direction: &Direction, distance: i32) -> Point {
    match direction {
        Direction::U => Point { x: starting_point.x, y: starting_point.y + distance },
        Direction::R => Point { x: starting_point.x + distance, y: starting_point.y },
        Direction::D => Point { x: starting_point.x, y: starting_point.y - distance },
        Direction::L => Point { x: starting_point.x - distance, y: starting_point.y }
    }
}

#[test]
fn test1() {
    let input = vec![
        "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
        "U62,R66,U55,R34,D71,R55,D58,R83".to_string()
    ];
    assert_eq!(159, run(input))
}

#[test]
fn test2() {
    let input = vec![
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()
    ];
    assert_eq!(135, run(input))
}