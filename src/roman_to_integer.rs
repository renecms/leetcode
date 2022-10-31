use std::{vec};

#[derive(Debug)]
enum RomanDigits {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl RomanDigits {
    fn value(&self) -> i32 {
        match *self {
            RomanDigits::I => 1,
            RomanDigits::V => 5,
            RomanDigits::X => 10,
            RomanDigits::L => 50,
            RomanDigits::C => 100,
            RomanDigits::D => 500,
            RomanDigits::M => 1000,
        }
    }

    pub fn from_string(char: &char) -> RomanDigits {
        match char.to_uppercase().next().unwrap() {
            'I' => RomanDigits::I,
            'V' => RomanDigits::V,
            'X' => RomanDigits::X,
            'L' => RomanDigits::L,
            'C' => RomanDigits::C,
            'D' => RomanDigits::D,
            'M' => RomanDigits::M,
            _ => panic!("invalid digit"),
        }
    }
}

fn _solution(s: String) -> i32 {
    let mut total: i32 = 0;

    let mut digits_stack: Vec<RomanDigits> = vec![];
    for char in s.chars() {
        let current_digit = RomanDigits::from_string(&char);
        if let Some(last_digit) = digits_stack.last() {

            if current_digit.value() < last_digit.value() {
                total += stack_subtraction(&mut digits_stack);
            } 
        } 
        digits_stack.push(current_digit);
    }

    if digits_stack.len() > 0 {
        total += stack_subtraction(&mut digits_stack);
    }
    total
}

fn stack_subtraction(stack: &mut Vec<RomanDigits>) -> i32 {
    let first_digit = stack.pop().unwrap();
    let mut sum: i32 = first_digit.value();
    
    while stack.len() > 0 {
        let current_digit = stack.pop().unwrap();
        if current_digit.value() < first_digit.value() {
            sum -= current_digit.value();
        } else {
            sum += current_digit.value();
        }
        
    }
    sum
}

mod tests {
    #[test]
    fn III_test() {
        assert_eq!(3, crate::roman_to_integer::_solution("III".to_string()));
    }

    #[test]
    fn LVIII_test() {
        assert_eq!(58, crate::roman_to_integer::_solution("LVIII".to_string()));
    }

    #[test]
    fn MCMXCIV_test() {
        assert_eq!(
            1994,
            crate::roman_to_integer::_solution("MCMXCIV".to_string())
        );
    }

    #[test]
    fn MCMXCIX_test() {
        assert_eq!(
            1999,
            crate::roman_to_integer::_solution("MCMXCIX".to_string())
        );
    }

    #[test]
    fn IIIX_test() {
        assert_eq!(
            7,
            crate::roman_to_integer::_solution("IIIX".to_string())
        );
    }
}
