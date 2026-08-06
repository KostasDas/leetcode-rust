use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut original = x;
        let mut digits = VecDeque::new();
        let mut n = 1;
        while original % 10 != original {
            digits.push_back(original % 10);
            n *= 10;
            original = original / 10;
        }
        digits.push_back(original);
        let mut result: i32 = 0;
        while let Some(v) = digits.pop_front() {
            let term = match v.checked_mul(n) {
                Some(t) => t,
                None => return 0,
            };
            result = match result.checked_add(term) {
                Some(s) => s,
                None => return 0,
            };
            n /= 10;
        }
        result
    }
}

fn main() {
    let x = 123;
    let res = Solution::reverse(x);
    println!("{}", res);
}
