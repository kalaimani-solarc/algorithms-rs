

pub fn letter_combinations(digits: &str) -> Vec<String> {
    let mut results = vec![];

    if digits.is_empty() {
        return results
    }

    let pad= ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

    let digits = digits.as_bytes();

    fn dfs(digits: &[u8], temp: String, index: usize, pad: &[&str], results: &mut Vec<String>) {
        if digits.len() == index {
            results.push(temp);
            return;
        }

        let lookup = (digits[index] - b'0') as usize;

        for letter in pad[lookup].chars() {
            let mut buf = String::with_capacity(temp.len() + 1);
            buf.push_str(&temp);
            buf.push(letter);
            dfs(digits, buf, index + 1, pad, results)
        }
    }

    dfs(digits.as_ref(), "".to_string(), 0, &pad, results.as_mut());

    results
}

#[cfg(test)]
mod tests {
    use crate::letter_combinations;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(Vec::<String>::new(), letter_combinations(""));
        assert_eq!(vec!["a", "b", "c"], letter_combinations("2"));
        assert_eq!(vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"], letter_combinations("23"));
    }
}