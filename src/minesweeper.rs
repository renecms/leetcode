fn _solution(matrix: Vec<Vec<bool>>) -> Vec<Vec<i32>> {
    let i_size = matrix.len() ;
    let j_size = matrix[0].len();

    let mut output_matrix: Vec<Vec<i32>> = vec![vec![0; j_size]; i_size];

    for i in 0..i_size {
        if let Some(row) = matrix.get(i) {
            for j in 0..j_size {
                if let Some(true) = row.get(j) {
                    output_matrix = _update_output_matrix(output_matrix, i, j);
                }
            }
        }
    }
    output_matrix
}

#[allow(clippy::needless_range_loop)]
fn _update_output_matrix(mut matrix: Vec<Vec<i32>>, i: usize, j: usize) -> Vec<Vec<i32>> {
    let lower_i_range = if i >= 1 { i - 1 } else { i };
    let higher_i_range =  if i < matrix.len() - 1 { i + 1 } else { matrix.len() - 1 };
    let lower_j_range = if j >= 1 { j - 1 } else { j };
    let higher_j_range = if j < matrix[0].len() - 1 { j + 1 } else { matrix[0].len() - 1 };

    for y in lower_i_range..=higher_i_range {
        for x in lower_j_range..=higher_j_range {
            if !(y == i && x == j) {
                matrix[y][x] += 1;
            }
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::_solution;

    #[test]
    fn minesweeper_3x3() {
        let matrix = vec![
            vec![true, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ];
        let answer = vec![vec![1, 2, 1], vec![2, 1, 1], vec![1, 1, 1]];
        assert_eq!(answer, _solution(matrix));
    }

    #[test]
    fn minesweeper_3x4() {
        let matrix = vec![
            vec![true, false, false, true],
            vec![false, false, true, false],
            vec![true, true, false, true],
        ];
        let answer = vec![vec![0, 2, 2, 1], vec![3, 4, 3, 3], vec![1, 2, 3, 1]];
        assert_eq!(answer, _solution(matrix));
    }
}
