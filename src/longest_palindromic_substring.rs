struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let bytes = s.as_bytes();
        let (mut start, mut len) = (0, 0);
        for i in 0..bytes.len() - 1 {
            let (r1, l1) = look_for_palindrome(bytes, i, i);
            let (r2, l2) = look_for_palindrome(bytes, i, i + 1);
            let len1 = l1 - r1 + 1;
            let len2 = l2 - r2 + 1;
            if len1 >= len2 && len1 > len {
                start = r1;
                len = len1;
            } else if len2 > len1 && len2 > len {
                start = r2;
                len = len2;
            }
        }
        String::from_utf8(bytes[start..(start + len)].to_vec()).unwrap()
    }
}

fn look_for_palindrome(b: &[u8], l: usize, r: usize) -> (usize, usize) {
    if l != r && b[l] != b[r] {
        return (0, 0);
    }
    let (mut left, mut right) = (l, r);
    while left > 0 && right < b.len() - 1 && b[left - 1] == b[right + 1] {
        left -= 1;
        right += 1;
    }
    (left, right)
}

#[test]
fn longest_palindrome_1() {
    assert_eq!(String::from("bcdcb"), Solution::longest_palindrome(String::from("abcdcbg")))
}

#[test]
fn longest_palindrome_2() {
    assert_eq!(String::from("a"), Solution::longest_palindrome(String::from("abc")))
}

#[test]
fn longest_palindrome_3() {
    assert_eq!(String::from("aa"), Solution::longest_palindrome(String::from("aa")))
}

#[test]
fn longest_palindrome_4() {
    assert_eq!(String::from(""), Solution::longest_palindrome(String::from("")))
}

#[test]
fn longest_palindrome_5() {
    assert_eq!(String::from("abccba"), Solution::longest_palindrome(String::from("abccba")))
}

#[test]
fn longest_palindrome_6() {
    assert_eq!(String::from("abcba"), Solution::longest_palindrome(String::from("abcba")))
}