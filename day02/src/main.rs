use std::fs::File;
use std::io::prelude::*;

fn main() {
    print!("{}", start_program(get_input_string()))
}

fn start_program(inputString: String) -> usize {
    let mut memory = generate_memory(inputString);

    memory[1] = 12;
    memory[2] = 2;

    run_program(&mut memory);
    
    memory[0]
}

fn run_program(memory: &mut Vec<usize>) {
    for x in 0..memory.len() {
        let op_position = x * 4;
        let operation = memory[op_position];
        if operation == 99 {
            break
        }
        let p1 = memory[memory[op_position + 1]];
        let p2 = memory[memory[op_position + 2]];
        let storage_position = memory[op_position + 3];
        print!("{} ", operation);
        print!("{} ", p1);
        print!("{} ", p2);
        match operation {
            1 => memory[storage_position] = p1 + p2,
            2 => memory[storage_position] = p1 * p2,
            _ => panic!("cenas")
        }
        print!("{}\n", memory[storage_position]);
    }
}

fn get_input_string() -> String {
    let mut file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/inputs/day02.txt")
        .expect("Couldn´t open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn generate_memory(inputString: String) -> Vec<usize> {
    inputString
        .split(",")
        .map(|l| l.parse::<usize>().expect("Couldn´t parse to usize"))
        .collect::<Vec<usize>>()
}

#[test]
fn t1() {
    assert_eq!(2, start_program("1,0,0,0,99".to_string()))
}

#[test]
fn t2() {
    assert_eq!(2, start_program("2,3,0,3,99".to_string()))
}

#[test]
fn t3() {
    assert_eq!(2, start_program("2,4,4,5,99,0".to_string()))
}

#[test]
fn t4() {
    assert_eq!(30, start_program("1,1,1,4,99,5,6,0,99".to_string()))
}

#[test]
fn t5() {
    assert_eq!(3500, start_program("1,9,10,3,2,3,11,0,99,30,40,50".to_string()))
}