use super::read_file;

pub fn split_rucksack(line: &str) -> (String, String) {
    let length = line.len() / 2;
    let first = line.chars().take(length).collect::<String>();
    let second = line.chars().skip(length).take(length).collect::<String>();

    (first, second)
}

pub fn find_repeated_char(line: &str) -> char {
    let (first, second) = split_rucksack(line);
    first.chars().find(|c| second.contains(*c)).unwrap()
}

pub fn evaluate_char_priority(c: char) -> i32 {
    let priority = c as i32 - 96;
    if priority < 0 {
        c as i32 - 64 + 26
    } else {
        priority
    }
}

pub fn get_priority_sum(filename: &str) -> i32 {
    let mut sum = 0;

    for line in read_file(filename) {
        let line = line.unwrap();
        let c = find_repeated_char(&line);
        let priority = evaluate_char_priority(c);
        sum += priority;
    }
    sum
}

mod tests {
    use super::*;

    #[test]
    fn test_get_priority_sum() {
        assert_eq!(get_priority_sum("input/day3.test"), 157);
    }

    #[test]
    fn test_evaluate_char_priority() {
        assert_eq!(evaluate_char_priority('A'), 27);
    }

    #[test]
    fn test_find_repeated_char() {
        assert_eq!(
            find_repeated_char("vJrwpWtwJgWrhcsFMMfFFhFp"),
            'p'
        );
    }

    #[test]
    fn test_split_rucksack() {
        assert_eq!(
            split_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"),
            ("vJrwpWtwJgWr".to_string(), "hcsFMMfFFhFp".to_string())
        );
    }

    #[test]
    fn test_file_read() {
        let lines = read_file("input/day3.test");
        assert_eq!(lines.count(), 6);
    }

}
