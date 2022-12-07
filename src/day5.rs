use std::{
    fs::File,
    io::{BufReader, Lines}, error::Error,
};

use super::read_file;

pub fn parse_state(mut layers: Vec<String>) -> Vec<String> {
    let mut initial_state: Vec<String> = Vec::new();

    // flip the layers to push onto the vector in the right order
    layers.reverse();

    // get number of stacks
    let stack_ct = (layers[0].len() + 1) / 4;

    // go through stacks and push to initial state vector as Strings
    for i in 0..stack_ct {
        initial_state.push(
            layers
                .iter()
                .map(|s| s.chars().nth(i * 4 + 1).unwrap())
                .collect::<String>()
                .trim()
                .to_string(),
        );
    }

    initial_state
}

pub fn parse_instructions(instructions: Vec<String>) -> Vec<[usize; 3]> {
    let mut moves: Vec<[usize; 3]> = Vec::new();

    for instruction in instructions {
        let temp: Vec<usize> = instruction
            .split_whitespace()
            .map(|s| s.parse().unwrap_or_default())
            .collect::<Vec<usize>>();
        moves.push([temp[1], temp[3], temp[5]]);
    }
    moves
}

pub fn parse_input(lines: Lines<BufReader<File>>) -> (Vec<String>, Vec<[usize; 3]>) {
    let mut state_chunk: Vec<String> = Vec::new();
    let mut instructions_chunk: Vec<String> = Vec::new();

    // split file into chunks and ditch any irrelevant lines
    for line in lines {
        let line = line.unwrap();

        if line.contains('[') {
            state_chunk.push(line);
        } else if line.starts_with("move") {
            instructions_chunk.push(line);
        } else {
            continue;
        }
    }

    // parse chunks individually
    let initial_state = parse_state(state_chunk);
    let instructions = parse_instructions(instructions_chunk);

    (initial_state, instructions)
}

pub fn move_crates(initial_state: Vec<String>, instructions: Vec<[usize; 3]>) -> Vec<String> {
    let mut state = initial_state
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for [number, from, to] in instructions {
        for _ in 0..number {
            let el = state[from - 1].pop().unwrap();
            state[to - 1].push(el);
        }
    }

    state
        .iter()
        .map(|v| v.iter().map(|&c| c.to_string()).collect::<String>())
        .collect::<Vec<String>>()
}

pub fn get_top_crates(final_state: Vec<String>) -> String {
    final_state
        .iter()
        .map(|s| s.chars().nth_back(0).unwrap().to_string())
        .collect::<String>()
}

pub fn get_part1_top_crates(filename: &str) -> Result<String, Box<dyn Error>> {
    // read in file
    let lines = read_file(filename)?;
    // parse input to get initial state and instructions
    let (initial_state, instructions) = parse_input(lines);
    // move crates
    let final_state = move_crates(initial_state, instructions);
    // take top crate of each stack
    Ok(get_top_crates(final_state))
}

pub fn move_crates_9001(initial_state: Vec<String>, instructions: Vec<[usize; 3]>) -> Vec<String> {
    let mut state = initial_state
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for [number, from, to] in instructions {
        let length = state[from - 1].len();
        let mut crates = state[from - 1].split_off(length - number);
        state[to - 1].append(&mut crates)
    }

    state
        .iter()
        .map(|v| v.iter().map(|&c| c.to_string()).collect::<String>())
        .collect::<Vec<String>>()
}

pub fn get_part2_top_crates(filename: &str) -> Result<String, Box<dyn Error>> {
    // read in file
    let lines = read_file(filename)?;
    // parse input to get initial state and instructions
    let (initial_state, instructions) = parse_input(lines);
    // move crates
    let final_state = move_crates_9001(initial_state, instructions);
    // take top crate of each stack
    Ok(get_top_crates(final_state))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_part2_top_crates() {
        assert_eq!(get_part2_top_crates("input/day5.test").unwrap(), "MCD")
    }

    #[test]
    fn test_move_crates_9001() {
        let initial_state = vec!["A", "BC", "D"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        let instructions = vec![[2, 2, 1]];

        let final_state = vec!["ABC", "", "D"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(move_crates_9001(initial_state, instructions), final_state);
    }

    #[test]
    fn test_get_part1_top_crates() {
        assert_eq!(get_part1_top_crates("input/day5.test").unwrap(), "CMZ")
    }

    #[test]
    fn test_get_top_crates() {
        let input = vec!["C", "M", "PDNZ"];
        let input_cast = input
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(get_top_crates(input_cast), "CMZ");
    }

    #[test]
    fn test_move_crates() {
        let initial_state = vec!["A", "BC", "D"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        let instructions = vec![[2, 2, 1]];

        let final_state = vec!["ACB", "", "D"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(move_crates(initial_state, instructions), final_state);
    }

    #[test]
    fn test_parse_input() {
        let lines = read_file("input/day5.test").unwrap();
        let output_initial_state = vec!["ZN", "MCD", "P"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        let output_instructions = vec![[1, 2, 1], [3, 1, 3], [2, 2, 1], [1, 1, 2]];
        assert_eq!(
            parse_input(lines),
            (output_initial_state, output_instructions)
        );
    }

    #[test]
    fn test_parse_instructions() {
        let lines = vec![
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        let output_moves = vec![[1, 2, 1], [3, 1, 3], [2, 2, 1], [1, 1, 2]];
        assert_eq!(parse_instructions(lines), output_moves);
    }

    #[test]
    fn test_parse_state() {
        let lines = vec!["    [D]     ", "[N] [C]     ", "[Z] [M] [P] "]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        let output_initial_state = vec!["ZN", "MCD", "P"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(parse_state(lines), output_initial_state);
    }

    #[test]
    fn test_read_file() {
        assert_eq!(read_file("input/day5.test").unwrap().count(), 9);
    }
}
