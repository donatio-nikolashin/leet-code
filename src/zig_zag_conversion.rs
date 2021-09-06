struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut rows: Vec<Vec<u8>> = Vec::new();
        for _num_row in 0..num_rows {
            rows.push(vec![])
        }
        let mut going_down = true;
        let mut num_row = 0;
        for b in s.as_bytes() {
            rows.get_mut(num_row).unwrap().push(*b);
            resolve_next_row(&num_rows, &mut going_down, &mut num_row);
        }
        let mut result = rows[0].to_vec();
        for index in 1..num_rows as usize {
            result.append(rows.get_mut(index).unwrap())
        }
        String::from_utf8(result).unwrap()
    }
}

fn resolve_next_row(num_rows: &i32, down: &mut bool, i: &mut usize) {
    if *num_rows < 2 { return; }
    let num_rows_usize = *num_rows as usize;
    if *down {
        if *i < num_rows_usize - 1 { *i += 1 } else {
            *i -= 1;
            *down = false
        }
    } else {
        if *i > 0 { *i -= 1; } else {
            *i += 1;
            *down = true;
        }
    }
}

#[test]
fn two_sum_test_1() {
    assert_eq!(String::from("PAHNAPLSIIGYIR"), Solution::convert(String::from("PAYPALISHIRING"), 3))
}

#[test]
fn two_sum_test_2() {
    assert_eq!(String::from("PINALSIGYAHRPI"), Solution::convert(String::from("PAYPALISHIRING"), 4))
}

#[test]
fn two_sum_test_3() {
    assert_eq!(String::from("A"), Solution::convert(String::from("A"), 1))
}