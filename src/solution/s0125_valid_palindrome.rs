use std::usize;

/**
 * [125] Valid Palindrome
 *
 * Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
 *
 * Note: For the purpose of this problem, we define empty string as valid palindrome.
 *
 * Example 1:
 *
 *
 * Input: "A man, a plan, a canal: Panama"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: "race a car"
 * Output: false
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-palindrome/
// discuss: https://leetcode.com/problems/valid-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }
        let s: Vec<char> = s.to_lowercase().chars().collect();
        let (mut i, mut j) = (0_usize, s.len() - 1);
        while i < j {
            while  i < j && !s[i].is_alphanumeric() {
                i += 1;
            }
            while i < j && !s[j].is_alphanumeric() {
                j -= 1;
            }
            // println!("{} {} {} {}",i,j,s[i],s[j]);
            if s[i] != s[j]{
                return false;
            }
            if i >= s.len() || j == 0 {
                break
            } else {
                i += 1;
                j -= 1;
            }
        }
        true
        // let mut trimmed = String::with_capacity(s.len());
        // for c in s.to_lowercase().chars().into_iter() {
        //     if c.is_alphanumeric() {
        //         trimmed.push(c);
        //     }
        // }
        // trimmed == trimmed.chars().rev().collect::<String>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_owned()), false);
        assert_eq!(Solution::is_palindrome("0P".to_owned()), false);
        assert_eq!(Solution::is_palindrome(".,".to_owned()), true);
        assert_eq!(Solution::is_palindrome("a,".to_owned()), true);
        assert_eq!(Solution::is_palindrome("a a".to_owned()), true);
    }
}
