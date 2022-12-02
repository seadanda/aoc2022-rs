use super::read_file;

pub fn get_elf_totals(filename: &str) -> Vec<i32> {
    let lines = read_file(filename);

    let mut elf_totals: Vec<i32> = Vec::new();
    let mut current_total: i32 = 0;

    for line in lines {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            // add to elf_totals
            elf_totals.push(current_total);
            current_total = 0;
        } else {
            current_total += line.parse::<i32>().unwrap();
        }
    }

    if current_total != 0 {
        elf_totals.push(current_total);
    }

    elf_totals
}

#[allow(dead_code)]
pub fn get_max(filename: &str) -> i32 {
    // find max value in elf_totals
    let elf_totals = get_elf_totals(filename);
    let mut max = 0;
    for elf_total in elf_totals {
        if elf_total > max {
            max = elf_total;
        }
    }
    max
}

#[allow(dead_code)]
pub fn get_top_three_sum(filename: &str) -> i32 {
    let mut elf_totals = get_elf_totals(filename);
    elf_totals.sort();
    elf_totals.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_top_three_sum() {
        assert_eq!(get_top_three_sum("input/day1.test"), 45000);
    }

    #[test]
    fn test_get_max() {
        assert_eq!(get_max("input/day1.test"), 24000);
    }

    #[test]
    fn test_get_elf_totals() {
        assert_eq!(
            get_elf_totals("input/day1.test"),
            vec![6000, 4000, 11000, 24000, 10000]
        );
    }

    #[test]
    fn test_read_file() {
        read_file("input/day1.test");
    }
}
