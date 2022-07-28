use std::{collections::HashMap, ops::Deref};

/**
 * [20] Valid Parentheses
 *
 * Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 * <ol>
 * 	Open brackets must be closed by the same type of brackets.
 * 	Open brackets must be closed in the correct order.
 * </ol>
 *
 * Note that an empty string is also considered valid.
 *
 * Example 1:
 *
 *
 * Input: "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: "(]"
 * Output: false
 *
 *
 * Example 4:
 *
 *
 * Input: "([)]"
 * Output: false
 *
 *
 * Example 5:
 *
 *
 * Input: "{[]}"
 * Output: true
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-parentheses/
// discuss: https://leetcode.com/problems/valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let m = std::collections::HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
        let mut stack: Vec<char> = vec![];
        for c in s.chars().into_iter() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            } else {
                match stack.pop() {
                    None => {
                        return false;
                    },
                    Some(p) => {
                        if c == *(m.get(&p).unwrap()) {
                            continue;
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
        stack.len() == 0
    }

    #[inline(always)]
    fn pair(open: char, close: char) -> bool {
        (open == '{' && close == '}')
            || (open == '(' && close == ')')
            || (open == '[' && close == ']')
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }
}
