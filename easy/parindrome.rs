// https://leetcode.com/problems/palindrome-number/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str: String = x.to_string();


    for i in 0..(x_str.len() / 2) {
        if x_str.chars().nth(i).unwrap() != x_str.chars().nth(x_str.len() - 1 - i).unwrap() {
            return false;
        }
    }
    return true
    }
}