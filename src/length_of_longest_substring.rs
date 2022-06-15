pub fn length_of_longest_substring(s: String) -> i32 {
    1
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
}