use substring::Substring;

/**
search a string for a chunk of specified length with no duplicated letters
*/
pub fn get_marker_pos(stream: &str, length: usize) -> usize {
    let mut pos = 0;

    for i in 0..stream.len() {
        if i < length - 1 {
            // cannot create a full chunk yet
            continue;
        }

        // check there are no duplicates in this chunk
        if !contains_duplicates(stream.substring(i + 1 - length, i + 1)) {
            pos = i;
            break;
        }
    }

    // return the character after this chunk
    pos + 1
}

/**
get string of arbitrary length and check for duplicates
*/
pub fn contains_duplicates(s: &str) -> bool {
    let length = s.len();
    let mut vec = s.chars().collect::<Vec<char>>();

    // dedupe
    vec.sort();
    vec.dedup();

    // check if any elements were removed
    vec.len() != length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicates() {
        assert!(contains_duplicates("pplf"));
        assert!(contains_duplicates("plpf"));
        assert!(contains_duplicates("plpp"));
        assert!(!contains_duplicates("pjlf"));
    }

    #[test]
    fn test_get_marker_pos() {
        assert_eq!(get_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(get_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(get_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(get_marker_pos("nnlpdvjthqldpwncqszvftbrmjlhg", 4), 5);
        assert_eq!(get_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(get_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_get_message_start_marker_pos() {
        assert_eq!(get_marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(get_marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(get_marker_pos("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(get_marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(get_marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
