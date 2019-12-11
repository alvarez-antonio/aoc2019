use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;

fn main() {
    print!("{:?}", part2())
}

fn part1() -> i32 {
    get_max(get_input_string(), vec![0,1,2,3,4])
}

fn part2() -> i32 {
    get_max_with_feedback(get_input_string(), vec![5,6,7,8,9])
}

fn get_max_with_feedback(input_string: String, signal_set: Vec<i32>) -> i32 {
    permutations(signal_set)
        .iter()
        .map({|sequence| (sequence, feedback_circuit(input_string.clone(), sequence))})
        .fold((&vec![0], -1231231),{ |max, result| 
            if result.1 > max.1 { 
                return result;
            } else { 
                return max;
            } 
        }).1.clone()
}

fn feedback_circuit(input_string: String, phase_sequence: &Vec<i32>) -> i32 {
    let mut amps = (phase_sequence)
        .into_iter()
        .map(|signal| (signal.clone(), generate_memory(input_string.clone()), 0))
        .collect::<Vec<(i32, Vec<i32>, i32)>>();
    
    // Start engines
    let mut thrust_signal = amps
        .iter_mut()
        .fold(0, { |previous_value, amp|
            run_program(amp.1.as_mut(), vec![amp.0.clone(), previous_value], &mut amp.2)
        });
    
    loop {
        let new_signal = amps
        .iter_mut()
            .fold(thrust_signal, { |previous_value, amp|
                run_program(amp.1.as_mut(), vec![previous_value], &mut amp.2)
            });
        if new_signal == std::i32::MIN {
            break;
        }
        thrust_signal = new_signal;
    }
    thrust_signal

}

fn get_max(input_string: String, signal_set: Vec<i32>) -> i32 {
    permutations(signal_set)
        .iter()
        .map({|sequence| (sequence, simple_circuit(input_string.clone(), sequence))})
        .fold((&vec![0], -1231231),{ |max, result| 
            if result.1 > max.1 { 
                return result;
            } else { 
                return max;
            } 
        }).1.clone()
}

fn permutations(input: Vec<i32>) -> Vec<Vec<i32>> {
    input.into_iter().permutations(5).collect_vec()
}

fn simple_circuit(input_string: String, phase_sequence: &Vec<i32>) -> i32 {
    phase_sequence
        .iter()
        .fold(0, { |previous_value, i|
            run_program(&mut generate_memory(input_string.clone()), vec![i.clone(), previous_value], &mut 0)
        })
}

fn run_program(memory: &mut Vec<i32>, input: Vec<i32>, pointer: &mut i32) -> i32 {
    let mut input_position = 0;
    let mut op_position = pointer.clone() as usize;
    loop {
        let instruction_string = format!("{:0>5}", memory[op_position]);
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
                memory[storage_position as usize] = input[input_position];
                print!("Input: {}\n", input[input_position]);
                op_position += 2; 
                input_position += 1;
            },
            "04" => {
                let (p1_storage_mode, _, _) = parse_storage_mode(instruction);
                let storage_position: i32 = if p1_storage_mode { memory[op_position + 1] } else { (op_position + 1) as i32 };
                print!("Output: {}\n", memory[storage_position as usize]);
                op_position += 2;
                *pointer = op_position as i32;
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
    print!("Output: {}\n", std::i32::MIN);
    *pointer = op_position as i32;
    std::i32::MIN
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
    let mut file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/inputs/day07.txt")
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
fn t06() {
    assert_eq!(1, run_program(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], vec![8], &mut 0))
}

#[test]
fn t07() {
    assert_eq!(0, run_program(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], vec![2], &mut 0))
}

#[test]
fn t08() {
    assert_eq!(0, run_program(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], vec![18], &mut 0))
}

#[test]
fn t09() {
    assert_eq!(1, run_program(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], vec![2], &mut 0))
}

#[test]
fn t10() {
    assert_eq!(0, run_program(&mut vec![3,3,1108,-1,8,3,4,3,99], vec![18], &mut 0))
}

#[test]
fn t11() {
    assert_eq!(1, run_program(&mut vec![3,3,1108,-1,8,3,4,3,99], vec![8], &mut 0))
}

#[test]
fn t12() {
    assert_eq!(0, run_program(&mut vec![3,3,1107,-1,8,3,4,3,99], vec![18], &mut 0))
}

#[test]
fn t13() {
    assert_eq!(1, run_program(&mut vec![3,3,1107,-1,8,3,4,3,99], vec![1], &mut 0))
}

#[test]
fn t14() {
    assert_eq!(0, run_program(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], vec![0], &mut 0))
}

#[test]
fn t15() {
    assert_eq!(1, run_program(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], vec![43], &mut 0))
}

#[test]
fn t16() {
    assert_eq!(0, run_program(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], vec![0], &mut 0))
}

#[test]
fn t17() {
    assert_eq!(1, run_program(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], vec![43], &mut 0))
}

#[test]
fn t18() {
    assert_eq!(1001, run_program(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], vec![43], &mut 0))
}

#[test]
fn t19() {
    assert_eq!(1000, run_program(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], vec![8], &mut 0))
}

#[test]
fn t20() {
    assert_eq!(999, run_program(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], vec![1], &mut 0))
}

#[test]
fn max_t1() {
    assert_eq!(43210, get_max("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string(), vec![0,1,2,3,4]))
}

#[test]
fn max_t2() {
    assert_eq!(54321, get_max("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string(), vec![0,1,2,3,4]))
}

#[test]
fn max_t3() {
    assert_eq!(65210, get_max("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string(), vec![0,1,2,3,4]))
}

#[test]
fn feedback_t1() {
    assert_eq!(139629729, get_max_with_feedback("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string(), vec![5,6,7,8,9]))
}

#[test]
fn feedback_t2() {
    assert_eq!(18216, get_max_with_feedback("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".to_string(), vec![5,6,7,8,9]))
}