use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("{}\n", get_result(find_layer_with_fewest_zero(split_layers((25, 6), to_array(get_input_string())))));
}

fn get_result(layer: Vec<char>) -> i32 {
    let one_count = layer.iter().fold(0, |acc, c| if *c == '1' { acc + 1 } else { acc });
    let two_count = layer.iter().fold(0, |acc, c| if *c == '2' { acc + 1 } else { acc });
    one_count * two_count
}

fn find_layer_with_fewest_zero(layers: Vec<Vec<char>>) -> Vec<char> {
    layers
        .iter()
        .map(|layer| (layer, layer.iter().fold(0, |acc, c| if *c == '0' { acc + 1 } else { acc })))
        .min_by_key(|x| x.1)
        .expect("found min")
        .0
        .clone()
}
 
fn split_layers(size: (i32, i32), input: Vec<char>) -> Vec<Vec<char>> {
    let layer_length = size.0 * size.1;
    input.chunks(layer_length as usize)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn to_array(input: String) -> Vec<char> {
    input.chars().collect()
}

fn get_input_string() -> String {
    let mut file = File::open("/Users/aalvarez/Projects/DiaCriativo/AdventOfCode2019/inputs/day08.txt")
        .expect("Couldn´t open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}