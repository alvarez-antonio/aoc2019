use std::collections::HashMap;

fn main() {
    let input = 109165..=576723;

    let quantity = input
        .into_iter()
        .filter(|number| matches_part2_criteria(*number))
        .count();

    print!("{}", quantity)
}

fn matches_part2_criteria(attempt: usize) -> bool {
    matches_part1_criteria(attempt) && has_strict_double(attempt)
}

fn matches_part1_criteria(attempt: usize) -> bool {
    has_duplicate_digits(attempt) && always_increasing(attempt)
}

fn has_duplicate_digits(attempt: usize) -> bool {
    
    let digit_0 = attempt / 100000;
    let digit_1 = (attempt - (digit_0 * 100000)) / 10000;
    let digit_2 = (attempt - (digit_0 * 100000 + digit_1 * 10000))/ 1000;
    let digit_3 = (attempt - (digit_0 * 100000 + digit_1 * 10000 + digit_2 * 1000))/ 100;
    let digit_4 = (attempt - (digit_0 * 100000 + digit_1 * 10000 + digit_2 * 1000 + digit_3 * 100))/ 10;
    let digit_5 = attempt - (digit_0 * 100000 + digit_1 * 10000 + digit_2 * 1000 + digit_3 * 100 + digit_4 * 10);

    print!("{} ", attempt);
    print!("{} ", digit_0);
    print!("{} ", digit_1);
    print!("{} ", digit_2);
    print!("{} ", digit_3);
    print!("{} ", digit_4);
    print!("{}\n", digit_5);

    digit_0 == digit_1 || 
    digit_1 == digit_2 || 
    digit_2 == digit_3 || 
    digit_3 == digit_4 || 
    digit_4 == digit_5
}

fn always_increasing(attempt: usize) -> bool {
    let digit_0 = attempt / 100000;
    let digit_1 = (attempt - (digit_0 * 100000)) / 10000;
    let digit_2 = (attempt - (digit_0 * 100000 + digit_1 * 10000))/ 1000;
    let digit_3 = (attempt - (digit_0 * 100000 + digit_1 * 10000 + digit_2 * 1000))/ 100;
    let digit_4 = (attempt - (digit_0 * 100000 + digit_1 * 10000 + digit_2 * 1000 + digit_3 * 100))/ 10;
    let digit_5 = attempt - (digit_0 * 100000 + digit_1 * 10000 + digit_2 * 1000 + digit_3 * 100 + digit_4 * 10);

    digit_0 <= digit_1 && 
    digit_1 <= digit_2 && 
    digit_2 <= digit_3 && 
    digit_3 <= digit_4 && 
    digit_4 <= digit_5
}

fn has_strict_double(attempt: usize) -> bool {
    let digit_0 = attempt / 100000;
    let digit_1 = (attempt - (digit_0 * 100000)) / 10000;
    let digit_2 = (attempt - (digit_0 * 100000 + digit_1 * 10000))/ 1000;
    let digit_3 = (attempt - (digit_0 * 100000 + digit_1 * 10000 + digit_2 * 1000))/ 100;
    let digit_4 = (attempt - (digit_0 * 100000 + digit_1 * 10000 + digit_2 * 1000 + digit_3 * 100))/ 10;
    let digit_5 = attempt - (digit_0 * 100000 + digit_1 * 10000 + digit_2 * 1000 + digit_3 * 100 + digit_4 * 10);
    let array = [digit_0, digit_1, digit_2, digit_3, digit_4, digit_5];
    let count_map: HashMap<usize, usize> = 
        (0..=9)
            .into_iter()
            .map(|digit| (digit, array.into_iter().filter(|array_digit| **array_digit == digit).count()))
            .collect();

    count_map.clone().values().any(|value| *value == 2)
}

fn not_in(array: Vec<usize>, key: usize) -> bool {
    array.into_iter().all(|k2| k2 < key)
}

#[test]
fn test1() {
    assert_eq!(true, matches_part1_criteria(111111))
}

#[test]
fn test2() {
    assert_eq!(false, matches_part1_criteria(223450))
}

#[test]
fn test3() {
    assert_eq!(false, matches_part1_criteria(123789))
}

#[test]
fn test4() {
    assert_eq!(true, matches_part2_criteria(112233))
}

#[test]
fn test5() {
    assert_eq!(false, matches_part2_criteria(123444))
}

#[test]
fn test6() {
    assert_eq!(true, matches_part2_criteria(111122))
}