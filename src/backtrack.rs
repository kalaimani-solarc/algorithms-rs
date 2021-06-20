use std::iter::FromIterator;
use std::ops::Deref;

/// (1) N QUEENS
///
/// ```
/// use algorithms::backtrack::solve_n_queens;
///
/// assert_eq!(
///     vec![
///         vec!["..Q.", "Q...", "...Q", ".Q.."],
///         vec![".Q..", "...Q", "Q...", "..Q."]
///     ],
///     solve_n_queens(4)
/// )
/// ```
pub fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    let mut board = vec![vec!['.'; n]; n];
    let mut result = vec![];

    fn dfs(board: &mut Vec<Vec<char>>, column: usize, result: &mut Vec<Vec<String>>) {
        if column == board.len() {
            result.push(build(board));
            return;
        }

        for row in 0..board.len() {
            if validate(board, row, column) {
                board[row][column] = 'Q';
                dfs(board, column + 1, result);
                board[row][column] = '.';
            }
        }
    }

    fn validate(board: &mut Vec<Vec<char>>, row: usize, column: usize) -> bool {
        for i in 0..board.len() {
            for j in 0..column {
                if board[i][j] == 'Q'
                    && (row + j == column + i || row + column == i + j || row == i)
                {
                    return false;
                }
            }
        }

        true
    }

    #[inline]
    fn build(board: &mut Vec<Vec<char>>) -> Vec<String> {
        board
            .iter()
            .map(|row| String::from_iter(row.iter()))
            .collect()
    }

    dfs(&mut board, 0, &mut result);

    result
}

/// (2) PALINDROME PARTITIONING
/// ```
/// use algorithms::backtrack::partition;
///
/// assert_eq!(vec![vec!["a","a","b"],vec!["aa","b"]], partition("aab".to_string()));
/// assert_eq!(vec![vec!["a"]], partition("a".to_string()));
/// ```
pub fn partition(st: String) -> Vec<Vec<String>> {
    fn dfs(index: usize, st: &str, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if index == st.len() {
            result.push(path.clone());
            return;
        }

        let chars: Vec<char> = st.chars().collect();

        for i in index..st.len() {
            if is_palindrome(chars.deref(), index, i) {
                path.push(st[index..(i + 1)].to_string());
                dfs(i + 1, st, path, result);
                path.pop();

            }
        }
    }

    fn is_palindrome(chars: &[char], mut start: usize, mut end: usize) -> bool {
        while start < end {
            if chars[start] != chars[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }

    let mut result = vec![];
    dfs(0, &st, &mut vec![], &mut result);
    result
}
