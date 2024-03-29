use std::iter::FromIterator;
use std::ops::Deref;


/// LETTER COMBINATIONS
///
/// Given a string containing digits from 2-9 inclusive, return all possible letter combinations
/// that the number could represent. Return the answer in any order.
///
/// A mapping of digit to letters (just like on the telephone buttons). Note that 1 does not map to any letters.
///
/// Examples:
/// ```
/// use algorithms::backtrack::letter_combinations;
///
/// assert_eq!(Vec::<String>::new(), letter_combinations(""));
/// assert_eq!(vec!["a", "b", "c"], letter_combinations("2"));
/// assert_eq!(vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"], letter_combinations("23"));
/// ```
///
pub fn letter_combinations(digits: &str) -> Vec<String> {
    let mut results = vec![];

    if digits.is_empty() {
        return results;
    }

    let pad = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];

    let digits = digits.as_bytes();

    fn dfs(digits: &[u8], buf: &mut String, index: usize, pad: &[&str], results: &mut Vec<String>) {
        if digits.len() == index {
            results.push(buf.to_owned());
            return;
        }

        let lookup = (digits[index] - b'0') as usize;

        for letter in pad[lookup].chars() {
            buf.push(letter);
            dfs(digits, buf, index + 1, pad, results);
            buf.pop();
        }
    }

    dfs(digits.as_ref(), &mut "".to_string(), 0, &pad, results.as_mut());

    results
}


/// N QUEENS
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

/// PALINDROME PARTITIONING
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

/// (3) GENERATE PARENTHESIS
/// ```
/// use algorithms::backtrack::generate_parenthesis;
///
/// assert_eq!(vec!["((()))", "(()())", "(())()", "()(())", "()()()"], generate_parenthesis(3));
/// assert_eq!(vec!["()"], generate_parenthesis(1));
/// ```
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn dfs(open: i32, close: i32, str: &mut String, n: i32, result: &mut Vec<String>) {
        if close > open || open > n {
            return;
        }

        if open == close && open == n {
            result.push(str.to_owned());
        }

        str.push_str("(");
        dfs(open + 1, close, str, n, result);
        str.pop();

        str.push_str(")");
        dfs(open, close + 1, str, n, result);
        str.pop();
    }

    let mut result = vec![];
    dfs(0, 0, &mut "".to_string(), n, &mut result);
    result
}

/// COMBINATION SUM I
///
/// ```
/// use algorithms::backtrack::combination_sum;
///
/// assert_eq!(vec![vec![2, 2, 3], vec![7]], combination_sum(vec![2, 3, 6, 7], 7));
/// assert_eq!(vec![vec![2, 2, 2, 2], vec![2, 3, 3],vec![3, 5]], combination_sum(vec![2, 3, 5], 8));
/// ```
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn dfs(candidates: &[i32], index: usize, temp: &mut Vec<i32>, target: i32, result: &mut Vec<Vec<i32>>) {
        let sum: i32 = temp.iter().sum();

        if sum > target {
            return;
        }

        if sum == target {
            result.push(temp.to_owned());
            return;
        }

        for i in index..candidates.len() {
            temp.push(candidates[i]);
            dfs(candidates, i, temp, target, result);
            temp.pop();
        }
    }

    let mut result = vec![];
    dfs(&candidates, 0, &mut vec![], target, &mut result);
    result
}

/// PERMUTATIONS
///
/// ```
/// use algorithms::backtrack::permute;
///
/// assert_eq!(
///     vec![
///         vec![1, 2, 3],
///         vec![1, 3, 2],
///         vec![2, 1, 3],
///         vec![2, 3, 1],
///         vec![3, 1, 2],
///         vec![3, 2, 1]
///     ],
///     permute(vec![1, 2, 3])
/// );
/// ```
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(nums: &[i32], index: usize, taken: &mut Vec<bool>, temp: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            result.push(temp.to_owned());
            return;
        }

        for (i, num) in nums.iter().enumerate() {
            if !taken[i] {
                taken[i] = true;
                temp.push(*num);
                dfs(nums, index + 1, taken, temp, result);
                temp.pop();
                taken[i] = false;
            }
        }
    }

    let mut result = vec![];
    let mut taken = vec![false; nums.len()];
    dfs(&nums, 0, &mut taken, &mut vec![], &mut result);
    result
}

/// SUBSETS
///
/// ```
/// use algorithms::backtrack::subsets;
///
/// assert_eq!(
///     vec![vec![], vec![3], vec![2], vec![2, 3], vec![1],vec! [1, 3],vec![1, 2], vec![1, 2, 3]],
///     subsets(vec![1, 2, 3])
/// );
///
/// assert_eq!(vec![vec![], vec![0]], subsets(vec![0]));
/// ```
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(nums: &[i32], index: usize, temp: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            result.push(temp.to_owned());
            return;
        }

        dfs(nums, index + 1, temp, result);
        temp.push(nums[index]);
        dfs(nums, index + 1, temp, result);
        temp.pop();
    }

    let mut result = vec![];
    dfs(&nums, 0, &mut vec![], &mut result);
    result
}