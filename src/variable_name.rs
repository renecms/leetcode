fn _solution(name: String) -> bool {
    for (index, char) in name.chars().enumerate() {
        if index == 0 && char.is_numeric() {
            return false;
        }
        if char != '_' && !char.is_alphabetic() && !char.is_alphanumeric() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::variable_name::_solution;

    #[test]
    fn variable_name() {
        assert!(_solution("var_1__Int".to_string()));
        assert!(!_solution("qq-q".to_string()));
        assert!(!_solution("2w2".to_string()));
    }
}