use std::{cmp, collections::HashMap};

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest: usize = 0;
    let mut substring_start_index = 0;
    let mut map: HashMap<char, usize> = HashMap::new();

    for (i, char) in s.chars().enumerate() {
        if let Some(value) = map.get(&char) {
            substring_start_index = cmp::max(substring_start_index, *value);
        }
        longest = cmp::max(longest, i - substring_start_index + 1);
        map.insert(char, i + 1);
    }

    longest as i32
}

#[cfg(test)]
mod tests {
    use crate::length_of_longest_substring::length_of_longest_substring;

    #[test]
    fn length_of_longest_substring_abcabcbb() {
        assert_eq!(3, length_of_longest_substring(String::from("abcabcbb")));
    }

    #[test]
    fn length_of_longest_substring_bbbbb() {
        assert_eq!(1, length_of_longest_substring(String::from("bbbbb")));
    }

    #[test]
    fn length_of_longest_substring_pwwkew() {
        assert_eq!(3, length_of_longest_substring(String::from("pwwkew")));
    }

    #[test]
    fn length_of_longest_substring_whitespace() {
        assert_eq!(1, length_of_longest_substring(String::from(" ")));
    }

    #[test]
    fn length_of_longest_substring_dvdf() {
        assert_eq!(3, length_of_longest_substring(String::from("dvdf")));
    }
}
