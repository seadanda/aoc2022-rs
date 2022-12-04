use super::read_file;

pub fn parse_series(s: &str) -> Vec<i32> {
    let bounds = s.split('-').collect::<Vec<&str>>();

    // surely there's a better way to cast from &str to i32...
    let bounds = bounds.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let bounds = bounds.iter().map(|s| s.parse::<i32>().unwrap().to_owned()).collect::<Vec<i32>>();
    
    (bounds[0]..bounds[1] + 1).collect::<Vec<i32>>()
}

pub fn is_intersecting_set(line: String) -> bool {
    // split line into first and second
    let line = line.split(',').collect::<Vec<&str>>();
    // parse both series
    let (first, second) = (parse_series(line[0]), parse_series(line[1]));
    // check if one is contained in the other
    first.iter().all(|i| second.contains(i)) || second.iter().all(|i| first.contains(i))
}

pub fn get_intersecting_sum(filename: &str) -> i32 {
    let lines = read_file(filename);
    let mut sum = 0;

    for line in lines {
        let line = line.unwrap();
        if is_intersecting_set(line) {
            sum += 1;
        }
    }
    sum
}


mod tests {
    use super::*;

    #[test]
    fn test_get_intersecting_sum() {
        assert_eq!(get_intersecting_sum("input/day4.test"), 2)
    }

    #[test]
    fn test_is_intersecting_set() {
        assert_eq!(is_intersecting_set("2-8,3-7".to_string()), true)
    }

    #[test]
    fn test_is_not_intersecting_set() {
        assert_eq!(is_intersecting_set("2-4,6-8".to_string()), false)
    }

    #[test]
    fn test_parse_series() {
        assert_eq!(parse_series("2-4"), vec!(2, 3, 4));
    }

    #[test]
    fn test_day4() {
        let lines = read_file("input/day4.test");
        assert_eq!(lines.count(), 6);
    }
}