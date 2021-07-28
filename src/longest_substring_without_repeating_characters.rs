use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max: usize = 0;
        let mut curr: usize = 0;
        let mut indices: HashMap<char, usize> = HashMap::new();
        for (loop_index, ch) in s.chars().enumerate() {
            let mut existing_index = indices.get(&ch);
            if existing_index.is_some() && loop_index - existing_index.unwrap() > curr {
                existing_index = None
            }
            match existing_index {
                None => {
                    curr += 1;
                    if curr > max {
                        max = curr;
                    }
                }
                Some(_) => {
                    curr = loop_index - existing_index.unwrap();
                }
            }
            indices.insert(ch, loop_index);
        }
        max as i32
    }
}

#[test]
fn length_of_longest_substring_1() {
    assert_eq!(3, Solution::length_of_longest_substring(String::from("abcca")))
}

#[test]
fn length_of_longest_substring_2() {
    assert_eq!(0, Solution::length_of_longest_substring(String::from("")))
}

#[test]
fn length_of_longest_substring_3() {
    assert_eq!(3, Solution::length_of_longest_substring(String::from("abaccabc")))
}

#[test]
fn length_of_longest_substring_4() {
    assert_eq!(5, Solution::length_of_longest_substring(String::from("tmmzuxt")))
}