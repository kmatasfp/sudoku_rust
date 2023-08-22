use std::collections::HashSet;

const ROWS: usize = 9;
const COLUMNS: usize = 9;

fn main() {
    println!("Hello, world!");
}

fn is_valid_solution(board: &[[u32; 9]; 9]) -> bool {
    // check if rows are correct
    for row in board {
        let mut row_set = HashSet::new();
        for n in row {
            row_set.insert(*n);
        }

        let sum: u32 = row_set.iter().sum();
        if sum != 45 || row_set.len() != 9 {
            return false;
        }
    }

    // check if columns are correct
    for j in 0..COLUMNS {
        let mut column_set = HashSet::new();
        for i in 0..ROWS {
            column_set.insert(board[i][j]);
        }

        let sum: u32 = column_set.iter().sum();
        if sum != 45 || column_set.len() != 9 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::is_valid_solution;

    #[test]
    fn is_valid_solution_should_return_false_in_case_rows_do_not_use_digits_1_to_9() {
        let board = [
            [1, 2, 3, 4, 5, 6, 7, 8, 10],
            [11, 12, 13, 14, 15, 16, 17, 18, 19],
            [21, 22, 23, 24, 25, 26, 27, 28, 29],
            [31, 32, 33, 34, 35, 36, 37, 38, 39],
            [41, 42, 43, 44, 45, 46, 47, 48, 49],
            [51, 52, 53, 54, 55, 56, 57, 58, 59],
            [61, 62, 63, 64, 65, 66, 67, 68, 69],
            [71, 72, 73, 74, 75, 76, 77, 78, 79],
            [81, 82, 83, 84, 85, 86, 87, 88, 89],
        ];
        assert_eq!(is_valid_solution(&board), false)
    }

    #[test]
    fn is_valid_solution_should_return_false_in_case_colums_do_not_use_digits_1_to_9() {
        let board = [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [12, 2, 3, 4, 5, 6, 7, 8, 9],
            [13, 2, 3, 4, 5, 6, 7, 8, 9],
            [14, 2, 3, 4, 5, 6, 7, 8, 9],
            [15, 2, 3, 4, 5, 6, 7, 8, 9],
            [16, 2, 3, 4, 5, 6, 7, 8, 9],
            [17, 2, 3, 4, 5, 6, 7, 8, 9],
            [18, 2, 3, 4, 5, 6, 7, 8, 9],
            [19, 2, 3, 4, 5, 6, 7, 8, 9],
        ];
        assert_eq!(is_valid_solution(&board), false)
    }

    #[test]
    fn is_valid_solution_should_return_false_in_case_rows_have_duplicate_numbers() {
        let board = [
            [1, 2, 3, 4, 5, 6, 7, 8, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 8],
            [1, 2, 3, 4, 5, 6, 7, 8, 8],
        ];
        assert_eq!(is_valid_solution(&board), false)
    }

    #[test]
    fn is_valid_solution_should_return_false_in_case_colums_have_duplicate_numbers() {
        let board = [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
        ];
        assert_eq!(is_valid_solution(&board), false)
    }

    #[test]
    fn is_valid_solution_should_return_true_in_case_of_valid_solution() {
        let board = [
            [5, 3, 9, 8, 7, 6, 4, 1, 2],
            [7, 2, 8, 3, 1, 4, 9, 6, 5],
            [6, 4, 1, 2, 9, 5, 7, 3, 8],
            [4, 6, 2, 5, 3, 9, 8, 7, 1],
            [3, 8, 5, 7, 2, 1, 6, 4, 9],
            [1, 9, 7, 4, 6, 8, 2, 5, 3],
            [2, 5, 6, 1, 8, 7, 3, 9, 4],
            [9, 1, 3, 6, 4, 2, 5, 8, 7],
            [8, 7, 4, 9, 5, 3, 1, 2, 6],
        ];

        assert_eq!(is_valid_solution(&board), true)
    }
}
