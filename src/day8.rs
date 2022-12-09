use super::read_file;
use super::E;

#[allow(dead_code)]
pub fn parse_to_2d_vec(filename: &str) -> Result<Vec<Vec<usize>>, E> {
    let lines = read_file(filename)?;
    Ok(lines
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect())
}

pub fn is_visible(forest: &Vec<Vec<usize>>, row: usize, col: usize) -> bool {
    let height = forest[row][col];
    let vertical = forest.iter().map(|r| r[col]).collect::<Vec<usize>>();
    let horizontal = forest[row].clone();

    !(vertical.iter().position(|&h| h >= height).unwrap() != row
        && vertical.iter().rposition(|&h| h >= height).unwrap() != row
        && horizontal.iter().position(|&h| h >= height).unwrap() != col
        && horizontal.iter().rposition(|&h| h >= height).unwrap() != col)
}

pub fn count_visible_trees(forest: Vec<Vec<usize>>) -> usize {
    let vlen = forest.len();
    let hlen = forest[0].len();

    let mut sum = 2 * (vlen + hlen) - 4;

    for i in 1..vlen - 1 {
        for j in 1..hlen - 1 {
            if is_visible(&forest, i, j) {
                sum += 1;
            }
        }
    }

    sum
}

pub fn check_forest_visibility(filename: &str) -> Result<usize, E> {
    let forest = parse_to_2d_vec(filename)?;
    Ok(count_visible_trees(forest))
}

pub fn get_scenic_score(forest: &Vec<Vec<usize>>, row: usize, col: usize) -> usize {
    let height = forest[row][col];
    let mut bottom: Vec<usize> = forest.iter().map(|r| r[col]).collect();
    let mut top: Vec<usize> = bottom.drain(0..row + 1).collect();
    let mut right = forest[row].clone();
    let mut left: Vec<usize> = right.drain(0..col + 1).collect();

    top.pop();
    left.pop();

    // reverse the order of trees out from the centre tree
    top.reverse();
    left.reverse();


    let bscore = match bottom.iter().position(|&h| h >= height) {
        Some(i) => i + 1,
        None => {
            if bottom.is_empty() {
                0
            } else {
                bottom.len()
            }
        }
    };
    let tscore = match top.iter().position(|&h| h >= height) {
        Some(i) => i + 1,
        None => {
            if top.is_empty() {
                0
            } else {
                top.len()
            }
        }
    };
    let rscore = match right.iter().position(|&h| h >= height) {
        Some(i) => i + 1,
        None => {
            if right.is_empty() {
                0
            } else {
                right.len()
            }
        }
    };
    let lscore = match left.iter().position(|&h| h >= height) {
        Some(i) => i + 1,
        None => {
            if left.is_empty() {
                0
            } else {
                left.len()
            }
        }
    };


    bscore * tscore * rscore * lscore
}

pub fn get_max_scenic_score(forest: Vec<Vec<usize>>) -> usize {
    // assuming rectangular forest
    let vlen = forest.len();
    let hlen = forest[0].len();

    let mut scenic_scores: Vec<usize> = Vec::new();

    for i in 0..vlen {
        for j in 0..hlen {
            scenic_scores.push(get_scenic_score(&forest, i, j));
        }
    }

    *scenic_scores.iter().reduce(|s1, s2| s1.max(s2)).unwrap()
}

pub fn find_max_scenic_score(filename: &str) -> Result<usize, E> {
    let forest = parse_to_2d_vec(filename)?;
    Ok(get_max_scenic_score(forest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_max_scenic_score() {
        let forest = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(get_max_scenic_score(forest), 8);
    }

    #[test]
    fn gets_scenic_score() {
        let forest = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(get_scenic_score(&forest, 1, 2), 4);
    }

    #[test]
    fn checks_forest_visibility() {
        assert_eq!(check_forest_visibility("input/day8.test").unwrap(), 21);
    }

    #[test]
    fn counts_visible_trees() {
        let forest = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(count_visible_trees(forest), 21);
    }

    #[test]
    fn checks_visibility() {
        let forest = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert!(is_visible(&forest, 1, 1));
    }

    #[test]
    fn parses_to_2d_vec() {
        let parsed_input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(parse_to_2d_vec("input/day8.test").unwrap(), parsed_input)
    }

    #[test]
    fn reads_file() {
        assert_eq!(read_file("input/day8.test").unwrap().count(), 5);
    }
}
