
pub fn _is_palindrome(x: i32) -> bool {
    let mut digits = vec![];
    let mut i = 0;
    let x = x as i64;
    if x < 0 {
        return false;
    }
    while (x / i64::pow(10, i)) > 0 {
        let current_digit = (x % i64::pow(10, i+1)) / i64::pow(10, i);
        digits.push(current_digit);
        i += 1;
    }

    let (digits, reverse_digits) = digits.split_at_mut((i/2) as usize);
    let mut reverse_digits = reverse_digits.iter().rev();
    
    for digit in digits {
        if let Some(reverse_digit) = reverse_digits.next() {
            if reverse_digit == digit {
                continue;
            }
            else {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::number_palindrome::_is_palindrome;

    #[test]
    fn number_palindromes() {
        assert!(_is_palindrome(121));
        assert!(!_is_palindrome(-121));
        assert!(_is_palindrome(1));
        assert!(!_is_palindrome(10));
        
    }

    #[test]
    fn palindrome_1410110141() {
        assert!(_is_palindrome(1410110141));
    }
}
