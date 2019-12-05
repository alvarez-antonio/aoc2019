use std::fs::File;
use std::io::prelude::*;

fn main() {
    print!("{}", part1_program(get_input_string()))
}

fn part1_program(input_string: String) -> i32 {
    let mut memory = generate_memory(input_string);

    run_program(&mut memory, 1)
}

fn part2_program(input_string: String) -> i32 {
    let original_memory = generate_memory(input_string);

    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = original_memory.to_vec();
            memory[1] = noun;
            memory[2] = verb;
            if run_program(&mut memory, 1) == 19690720 {
                print!("{} ", noun);
                print!("{} ", verb);
                return 100 * noun + verb
            }
        }
    }

    panic!("Morreu.");
}

fn run_program(memory: &mut Vec<i32>, input: i32) -> i32 {
    let mut op_position = 0;
    loop {
        let instruction_string = format!("{:0>5}",memory[op_position]);
        let instruction = instruction_string.as_str();
        let (p1_storage_mode, p2_storage_mode, ) = parse_instruction(instruction);
        print!("opcode {}\n", instruction);
        match &instruction[3..5] {
            "01" => {
                let p1 = if p1_storage_mode { 
                    memory[memory[op_position + 1 as usize] as usize]
                } else {
                    memory[op_position + 1 as usize]
                };
                let p2 = if p2_storage_mode { 
                    memory[memory[op_position + 2 as usize] as usize]
                } else {
                    memory[op_position + 2 as usize]
                };
                let storage_position = memory[op_position + 3];
                memory[storage_position as usize] = p1 + p2;
                op_position += 4; 
            },
            "02" => {
                let p1 = if p1_storage_mode { 
                    memory[memory[op_position + 1 as usize] as usize]
                } else {
                    memory[op_position + 1 as usize]
                };
                let p2 = if p2_storage_mode { 
                    memory[memory[op_position + 2 as usize] as usize]
                } else {
                    memory[op_position + 2 as usize]
                };
                let storage_position = memory[op_position + 3];
                memory[storage_position as usize] = p1 * p2;
                op_position += 4; 
            },
            "03" => {
                let storage_position = memory[op_position + 1];
                memory[storage_position as usize] = input;
                op_position += 2; 
            },
            "04" => {
                let storage_position = memory[op_position + 1];
                print!("Output: {}\n", memory[storage_position as usize]);
                op_position += 2; 
            },
            "99" => break,
            _ => panic!("cenas")
        }
    }

    memory[0]
}

fn parse_instruction(instruction: &str) -> (bool, bool) {
    (
        instruction.chars().nth(2).unwrap() == '0', 
        instruction.chars().nth(1).unwrap() == '0')
}

fn get_input_string() -> String {
    let mut file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/inputs/day05.txt")
        .expect("Couldn´t open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn generate_memory(inputString: String) -> Vec<i32> {
    inputString
        .split(",")
        .map(|l| l.parse::<i32>().expect("Couldn´t parse to usize"))
        .collect::<Vec<i32>>()
}

#[test]
fn t1() {
    assert_eq!(2, run_program(&mut vec![1,0,0,0,99], 1))
}

#[test]
fn t2() {
    assert_eq!(2, run_program(&mut vec![2,3,0,3,99], 1))
}

#[test]
fn t3() {
    assert_eq!(2, run_program(&mut vec![2,4,4,5,99,0], 1))
}

#[test]
fn t4() {
    assert_eq!(30, run_program(&mut vec![1,1,1,4,99,5,6,0,99], 1))
}

#[test]
fn t5() {
    assert_eq!(3500, run_program(&mut vec![1,9,10,3,2,3,11,0,99,30,40,50], 1))
}