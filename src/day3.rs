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

pub fn find_shared_char(group: Vec<String>) -> char {
    group[0]
        .chars()
        .find(|c| group.iter().all(|s| s.contains(*c)))
        .unwrap()
}

pub fn get_group_priority_sum(filename: &str) -> i32 {
    let mut sum = 0;

    let lines = read_file(filename)
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    for i in 0..lines.len() / 3 {
        let line_trio = lines
            .iter()
            .skip(i * 3)
            .take(3)
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let c = find_shared_char(line_trio);
        let priority = evaluate_char_priority(c);
        sum += priority;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_group_priority_sum() {
        assert_eq!(get_group_priority_sum("input/day3.test"), 70);
    }

    #[test]
    fn test_get_priority_sum() {
        assert_eq!(get_priority_sum("input/day3.test"), 157);
    }

    #[test]
    fn test_evaluate_char_priority_lower() {
        assert_eq!(evaluate_char_priority('p'), 16);
    }

    #[test]
    fn test_evaluate_char_priority_capital() {
        assert_eq!(evaluate_char_priority('A'), 27);
    }

    #[test]
    fn test_find_repeated_char() {
        assert_eq!(find_repeated_char("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
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
