fn solution(input_array: Vec<i32>, elem_to_replace: i32, substitution_elem: i32) -> Vec<i32> {
    let mut result_vec = vec![];
    for num in input_array {
        if num == elem_to_replace {
            result_vec.push(substitution_elem);
        } else {
            result_vec.push(num);
        }
    }
    result_vec
}

mod tests {
    #[test]
    fn replace_test() {
        assert_eq!(vec![3,2,3], crate::replace::solution(vec![1,2,1], 1, 3));
    }
}