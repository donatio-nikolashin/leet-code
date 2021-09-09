struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let min_value = i32::MIN as i64;
        let max_value = i32::MAX as i64;
        let mut result: i64 = 0;
        let mut digit_found = false;
        let mut sign_found = false;
        let mut positive = true;
        for ch in s.chars() {
            if ch.is_digit(10) {
                digit_found = true;
                result *= 10;
                result += ch.to_digit(10).unwrap() as i64;
                if positive && result > max_value {
                    return i32::MAX;
                } else if !positive && -result < min_value {
                    return i32::MIN;
                }
            } else {
                if digit_found {
                    return if positive { result as i32 } else { -result as i32 };
                } else {
                    if ch == ' ' && !sign_found { continue; } else if !sign_found {
                        if ch == '+' {
                            sign_found = true;
                        } else if ch == '-' {
                            sign_found = true;
                            positive = false
                        } else {
                            return result as i32;
                        }
                    } else {
                        return result as i32;
                    }
                }
            }
        }
        return if positive { result as i32 } else { -result as i32 };
    }
}

#[test]
fn my_atoi_test_1() {
    assert_eq!(123, Solution::my_atoi(String::from("123")))
}

#[test]
fn my_atoi_test_2() {
    assert_eq!(-123, Solution::my_atoi(String::from("-123")))
}

#[test]
fn my_atoi_test_3() {
    assert_eq!(-123, Solution::my_atoi(String::from("  -123")))
}

#[test]
fn my_atoi_test_4() {
    assert_eq!(-123, Solution::my_atoi(String::from("  -123safd")))
}

#[test]
fn my_atoi_test_5() {
    assert_eq!(0, Solution::my_atoi(String::from("w  123safd")))
}

#[test]
fn my_atoi_test_6() {
    assert_eq!(2147483647, Solution::my_atoi(String::from("100000000000")))
}

#[test]
fn my_atoi_test_7() {
    assert_eq!(-2147483648, Solution::my_atoi(String::from("-100000000000")))
}