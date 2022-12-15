use super::read_file;
use super::E;
use geo::{coord, Coord as OtherCoord};
use num_traits::Zero;

type Coord = OtherCoord<i32>;
type Coords = Vec<Coord>;

pub fn parse_input(filename: &str) -> Result<Coords, E> {
    let input = read_file(filename)?;
    let mut input_parsed: Coords = Vec::new();

    for line in input {
        let line = line?;
        let el = match line.chars().next().unwrap() {
            'R' => coord! {x: line.split(' ').last().unwrap().parse::<i32>()?, y: 0},
            'L' => coord! {x: -line.split(' ').last().unwrap().parse::<i32>()?, y: 0},
            'U' => coord! {x: 0, y: line.split(' ').last().unwrap().parse::<i32>()?},
            'D' => coord! {x: 0, y: -line.split(' ').last().unwrap().parse::<i32>()?},
            _ => panic!(),
        };
        input_parsed.push(el);
    }
    Ok(input_parsed)
}

pub fn evaluate_step(
    head_position: Coord,
    tail_position: Coord,
    instruction: Coord,
) -> (Coord, Coord, Coords) {
    let mut tail_positions = vec![tail_position];
    let mut new_head = head_position;
    let mut new_tail = tail_position;

    let (x, y) = instruction.x_y();
    for _ in 1..(x.abs() + y.abs() + 1) {
        let change: Coord =
            coord! {x: x.checked_div(x.abs()).unwrap_or(0), y: y.checked_div(y.abs()).unwrap_or(0)};
        new_head = new_head + change;

        let (xt, yt) = new_tail.x_y();
        let (xh, yh) = new_head.x_y();

        let xdiff = xh - xt;
        let ydiff = yh - yt;

        if [0, 1].contains(&xdiff.abs()) && [0, 1].contains(&ydiff.abs()) {
            continue;
        }
        let xnew = match xdiff {
            x if x > 0 => xt + 1,
            x if x < 0 => xt - 1,
            _ => xt,
        };

        let ynew = match ydiff {
            y if y > 0 => yt + 1,
            y if y < 0 => yt - 1,
            _ => yt,
        };

        new_tail = coord! {x: xnew, y: ynew};
        tail_positions.push(new_tail);
    }

    (new_head, new_tail, tail_positions)
}

pub fn get_number_of_tail_positions(instructions: Coords) -> usize {
    let mut head_position: Coord = Zero::zero();
    let mut tail_position: Coord = Zero::zero();
    let mut previous_positions: Coords = Vec::new();

    for instruction in instructions {
        let step_positions;
        (head_position, tail_position, step_positions) =
            evaluate_step(head_position, tail_position, instruction);

        for step_position in step_positions {
            if !previous_positions.contains(&step_position) {
                previous_positions.push(step_position);
            }
        }
    }
    
    previous_positions.len()
}

pub fn count_tail_positions(filename: &str) -> Result<usize,E> {
    let instructions = parse_input(filename)?;
    Ok(get_number_of_tail_positions(instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_tail_positions() {
        assert_eq!(count_tail_positions("input/day9.test").unwrap(), 13);
    }

    #[test]
    fn gets_number_of_tail_positions() {
        let input_parsed = vec![
            Coord::from([4, 0]),
            Coord::from([0, 4]),
            Coord::from([-3, 0]),
            Coord::from([0, -1]),
            Coord::from([4, 0]),
            Coord::from([0, -1]),
            Coord::from([-5, 0]),
            Coord::from([2, 0]),
        ];
        assert_eq!(get_number_of_tail_positions(input_parsed), 13);
    }

    #[test]
    fn evaluates_step() {
        let head_pos: Coord = Zero::zero();
        let tail_pos: Coord = Zero::zero();
        let instruction = Coord::from([4, 0]);
        let (new_head, new_tail, positions) = evaluate_step(head_pos, tail_pos, instruction);

        let expected_head = Coord::from([4, 0]);
        let expected_tail = Coord::from([3, 0]);
        let expected_positions = vec![
            Coord::from([0, 0]),
            Coord::from([1, 0]),
            Coord::from([2, 0]),
            Coord::from([3, 0]),
        ];

        assert_eq!(new_head, expected_head);
        assert_eq!(new_tail, expected_tail);
        assert_eq!(positions, expected_positions);
    }

    #[test]
    fn parses_input() {
        let filename = "input/day9.test";
        let input = parse_input(filename).unwrap();
        let input_parsed = vec![
            Coord::from([4, 0]),
            Coord::from([0, 4]),
            Coord::from([-3, 0]),
            Coord::from([0, -1]),
            Coord::from([4, 0]),
            Coord::from([0, -1]),
            Coord::from([-5, 0]),
            Coord::from([2, 0]),
        ];
        assert_eq!(input, input_parsed);
    }

    #[test]
    fn reads_file() {
        assert_eq!(read_file("input/day9.test").unwrap().count(), 8);
    }
}
