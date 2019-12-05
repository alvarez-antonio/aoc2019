use std::fs::File;
use std::io::prelude::*;

fn main() {
    print!("{}", start_program(get_input_string()))
}

fn start_program(input_string: String) -> i32 {
    let mut memory = generate_memory(input_string);

    run_program(&mut memory, 5)
}

fn run_program(memory: &mut Vec<i32>, input: i32) -> i32 {
    let mut op_position = 0;
    loop {
        let instruction_string = format!("{:0>5}",memory[op_position]);
        let instruction = instruction_string.as_str();
        print!("{:?}\n", memory);
        print!("opcode {}\n", instruction);
        match &instruction[3..5] {
            "01" => {
                let (p1, p2, p3) = get_parameters(instruction, &memory, op_position);
                memory[p3 as usize] = p1 + p2;
                op_position += 4; 
            },
            "02" => {
                let (p1, p2, p3) = get_parameters(instruction, &memory, op_position);
                memory[p3 as usize] = p1 * p2;
                op_position += 4; 
            },
            "03" => {
                let storage_position = memory[op_position + 1];
                memory[storage_position as usize] = input;
                op_position += 2; 
            },
            "04" => {
                let storage_position = memory[op_position + 1];
                return memory[storage_position as usize];
            },
            "05" => {
                let (p1, p2, _) = get_parameters(instruction, &memory, op_position);
                if p1 != 0 {
                    op_position = p2 as usize;
                } else {
                    op_position += 3;
                }
            },
            "06" => {
                let (p1, p2, _) = get_parameters(instruction, &memory, op_position);
                if p1 == 0 {
                    op_position = p2 as usize;
                } else {
                    op_position += 3;
                }
            }
            "07" => {
                let (p1, p2, p3) = get_parameters(instruction, &memory, op_position);
                memory[p3 as usize] = if p1 < p2 { 1 } else { 0 };
                op_position += 4;
            }
            "08" => {
                let (p1, p2, p3) = get_parameters(instruction, &memory, op_position);
                memory[p3 as usize] = if p1 == p2 { 1 } else { 0 };
                op_position += 4;
            }
            "99" => break,
            _ => panic!("cenas")
        }
    }

    memory[0]
}

fn get_parameters(instruction: &str, memory: &Vec<i32>, op_position: usize) -> (i32, i32, i32) {
    let (p1_storage_mode, p2_storage_mode, _) = parse_storage_mode(instruction);
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
    let p3 = memory[op_position + 3 as usize];

    (p1, p2, p3)
}

fn parse_storage_mode(instruction: &str) -> (bool, bool, bool) {
    (
        instruction.chars().nth(2).unwrap() == '0', 
        instruction.chars().nth(1).unwrap() == '0',
        instruction.chars().nth(0).unwrap() == '0'
    )
}

fn get_input_string() -> String {
    let mut file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/inputs/day05.txt")
        .expect("Couldn´t open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn generate_memory(input_string: String) -> Vec<i32> {
    input_string
        .split(",")
        .map(|l| l.parse::<i32>().expect("Couldn´t parse to usize"))
        .collect::<Vec<i32>>()
}

#[test]
fn t01() {
    assert_eq!(2, run_program(&mut vec![1,0,0,0,99], 1))
}

#[test]
fn t02() {
    assert_eq!(2, run_program(&mut vec![2,3,0,3,99], 1))
}

#[test]
fn t03() {
    assert_eq!(2, run_program(&mut vec![2,4,4,5,99,0], 1))
}

#[test]
fn t04() {
    assert_eq!(30, run_program(&mut vec![1,1,1,4,99,5,6,0,99], 1))
}

#[test]
fn t05() {
    assert_eq!(3500, run_program(&mut vec![1,9,10,3,2,3,11,0,99,30,40,50], 1))
}

#[test]
fn t06() {
    assert_eq!(1, run_program(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], 8))
}

#[test]
fn t07() {
    assert_eq!(0, run_program(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], 2))
}

#[test]
fn t08() {
    assert_eq!(0, run_program(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 18))
}

#[test]
fn t09() {
    assert_eq!(1, run_program(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 2))
}

#[test]
fn t10() {
    assert_eq!(0, run_program(&mut vec![3,3,1108,-1,8,3,4,3,99], 18))
}

#[test]
fn t11() {
    assert_eq!(1, run_program(&mut vec![3,3,1108,-1,8,3,4,3,99], 8))
}

#[test]
fn t12() {
    assert_eq!(0, run_program(&mut vec![3,3,1107,-1,8,3,4,3,99], 18))
}

#[test]
fn t13() {
    assert_eq!(1, run_program(&mut vec![3,3,1107,-1,8,3,4,3,99], 1))
}

#[test]
fn t14() {
    assert_eq!(0, run_program(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0))
}

#[test]
fn t15() {
    assert_eq!(1, run_program(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 43))
}

#[test]
fn t16() {
    assert_eq!(0, run_program(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0))
}

#[test]
fn t17() {
    assert_eq!(1, run_program(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 43))
}

#[test]
fn t18() {
    assert_eq!(1001, run_program(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 43))
}

#[test]
fn t19() {
    assert_eq!(1000, run_program(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8))
}

#[test]
fn t20() {
    assert_eq!(999, run_program(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 5))
}