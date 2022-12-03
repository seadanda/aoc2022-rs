use super::read_file;

pub fn get_line_score(line: &str) -> i32 {
    let mut score = 0;
    if line.ends_with('X') {
        score += 1;
        if line.starts_with('A') {
            score += 3;
        } else if line.starts_with('B') {
            score += 0;
        } else if line.starts_with('C') {
            score += 6;
        }
    } else if line.ends_with('Y') {
        score += 2;
        if line.starts_with('A') {
            score += 6;
        } else if line.starts_with('B') {
            score += 3;
        } else if line.starts_with('C') {
            score += 0;
        }
    } else if line.ends_with('Z') {
        score += 3;
        if line.starts_with('A') {
            score += 0;
        } else if line.starts_with('B') {
            score += 6;
        } else if line.starts_with('C') {
            score += 3;
        }
    }
    score
}

pub fn get_corrected_line_score(line: &str) -> i32 {
    let mut score = 0;
    if line.starts_with('A') {
        if line.ends_with('X') {
            score += 3;
        } else if line.ends_with('Y') {
            score += 4;
        } else if line.ends_with('Z') {
            score += 8;
        }
    } else if line.starts_with('B') {
        if line.ends_with('X') {
            score += 1;
        } else if line.ends_with('Y') {
            score += 5;
        } else if line.ends_with('Z') {
            score += 9;
        }
    } else if line.starts_with('C') {
        if line.ends_with('X') {
            score += 2;
        } else if line.ends_with('Y') {
            score += 6;
        } else if line.ends_with('Z') {
            score += 7;
        }
    }
    score
}

#[allow(dead_code)]
pub fn get_score(filename: &str) -> i32 {
    let lines = read_file(filename);
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        let score = get_line_score(&line);
        total += score;
    }
    total
}

#[allow(dead_code)]
pub fn get_corrected_score(filename: &str) -> i32 {
    let lines = read_file(filename);
    let mut total = 0;
    for line in lines {
        let line = line.unwrap();
        let score = get_corrected_line_score(&line);
        total += score;
    }
    total
}

mod tests {
    use super::*;

    #[test]
    fn test_get_corrected_score() {
        assert_eq!(get_corrected_score("input/day2.txt"), 12);
    }

    #[test]
    fn test_get_corrected_line_score() {
        assert_eq!(get_corrected_line_score("A Y"), 4);
    }

    #[test]
    fn test_get_score() {
        assert_eq!(get_score("input/day2.test"), 15);
    }

    #[test]
    fn test_get_line_score() {
        assert_eq!(get_line_score("A Y"), 8);
    }

    #[test]
    fn test_read_file() {
        let f = "input/day2.test";
        let lines = read_file(f);
        assert_eq!(lines.count(), 3);
    }
}
