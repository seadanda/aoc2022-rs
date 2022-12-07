use std::error::Error;

use super::read_file;

pub fn parse_series(s: &str) -> Vec<i32> {
    let bounds = s.split('-').collect::<Vec<&str>>();

    // surely there's a better way to cast from &str to i32...
    let bounds = bounds.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let bounds = bounds.iter().map(|s| s.parse::<i32>().unwrap().to_owned()).collect::<Vec<i32>>();
    
    (bounds[0]..bounds[1] + 1).collect::<Vec<i32>>()
}

pub fn is_intersecting_set(line: &str) -> bool {
    // split line into first and second
    let line = line.split(',').collect::<Vec<&str>>();
    // parse both series
    let (first, second) = (parse_series(line[0]), parse_series(line[1]));
    // check if one is contained in the other
    first.iter().all(|i| second.contains(i)) || second.iter().all(|i| first.contains(i))
}

pub fn is_partial_intersecting_set(line: &str) -> bool {
    // split line into first and second
    let line = line.split(',').collect::<Vec<&str>>();
    // parse both series
    let (first, second) = (parse_series(line[0]), parse_series(line[1]));
    // check if one is partially contained in the other
    let test = first.iter().find(|&i| second.contains(i));
    test.is_some()
}

pub fn get_intersecting_sum(filename: &str, allow_partial: bool) -> Result<i32, Box<dyn Error>> {
    let lines = read_file(filename)?;
    let mut sum = 0;

    for line in lines {
        let line = line?;
        if (allow_partial && is_partial_intersecting_set(&line)) || is_intersecting_set(&line) {
            sum += 1;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_partial_intersecting_sum() {
        assert_eq!(get_intersecting_sum("input/day4.test", true).unwrap(), 4)
    }

    #[test]
    fn test_is_partial_intersecting_set() {
        assert!(is_partial_intersecting_set("2-8,6-9"))
    }

    #[test]
    fn test_is_not_partial_intersecting_set() {
        assert!(!is_partial_intersecting_set("2-4,6-8"))
    }

    #[test]
    fn test_get_intersecting_sum() {
        assert_eq!(get_intersecting_sum("input/day4.test", false).unwrap(), 2)
    }

    #[test]
    fn test_is_intersecting_set() {
        assert!(is_intersecting_set("2-8,3-7"))
    }

    #[test]
    fn test_is_not_intersecting_set() {
        assert!(!is_intersecting_set("2-4,6-8"))
    }

    #[test]
    fn test_parse_series() {
        assert_eq!(parse_series("2-4"), vec!(2, 3, 4));
    }

    #[test]
    fn test_day4() {
        let lines = read_file("input/day4.test").unwrap();
        assert_eq!(lines.count(), 6);
    }
}