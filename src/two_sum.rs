use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices:HashMap<i32, i32> = HashMap::new();
        for (index, element) in nums.iter().enumerate() {
            let diff = target - element;
            if indices.contains_key(&diff) {
                return vec![*indices.get(&diff).unwrap(), index as i32]
            } else {
                indices.insert(*element, index as i32);
            }
        }
        panic!("No solution found")
    }
}

#[test]
fn two_sum_test_1() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9))
}

#[test]
fn two_sum_test_2() {
    assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6))
}

#[test]
fn two_sum_test_3() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6))
}

#[test]
#[should_panic]
fn two_sum_test_panic() {
    let _solution = Solution::two_sum(vec![1, 2, 3], 6);
}