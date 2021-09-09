struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str = x.to_string();
        let bytes = str.as_bytes();
        for i in 0..bytes.len() / 2 {
            if bytes[i] != bytes[bytes.len() - i - 1] {
                return false
            }
        }
        return true
    }
}

// simple solution:
// impl Solution {
//     pub fn is_palindrome(x: i32) -> bool {
//         let str = x.to_string();
//         str == str.chars().rev().collect::<String>()
//     }
// }

#[test]
fn is_palindrome_test_1() {
    assert_eq!(true, Solution::is_palindrome(121))
}