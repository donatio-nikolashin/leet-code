struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let merged_size: usize = nums1.len() + nums2.len();
        if merged_size == 0 {
            return 0.0;
        }
        let even: bool = merged_size % 2 == 0;
        let mut pointer1: usize = 0;
        let mut pointer2: usize = 0;
        let mut pos = 0;
        let mut curr = 0;
        while even && pos <= (merged_size / 2) - 1 || !even && pos <= merged_size / 2 {
            curr = next_value(&nums1, &nums2, &mut pointer1, &mut pointer2);
            pos += 1
        }
        return if even {
            (f64::from(curr) + f64::from(next_value(&nums1, &nums2, &mut pointer1, &mut pointer2))) / 2.0
        } else {
            f64::from(curr)
        } as f64;
    }
}

fn next_value(nums1: &Vec<i32>, nums2: &Vec<i32>, pointer1: &mut usize, pointer2: &mut usize)
    -> i32 {
    let first = nums1.get(*pointer1);
    let second = nums2.get(*pointer2);
    return if first.is_none() && second.is_some() {
        *pointer2 += 1;
        *second.unwrap()
    } else if second.is_none() && first.is_some() {
        *pointer1 += 1;
        *first.unwrap()
    } else if second.unwrap() >= first.unwrap() {
        *pointer1 += 1;
        *first.unwrap()
    } else {
        *pointer2 += 1;
        *second.unwrap()
    };
}

#[test]
fn find_median_sorted_arrays_1() {
    assert_eq!(1.0, Solution::find_median_sorted_arrays(vec![1], vec![]))
}

#[test]
fn find_median_sorted_arrays_2() {
    assert_eq!(1.5, Solution::find_median_sorted_arrays(vec![1], vec![2]))
}

#[test]
fn find_median_sorted_arrays_3() {
    assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![1], vec![2, 3]))
}

#[test]
fn find_median_sorted_arrays_4() {
    assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![], vec![2, 3]))
}

