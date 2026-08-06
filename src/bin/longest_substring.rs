use std::collections::HashMap;

struct Solution;

/// Given a string s, find the length of the longest without duplicate characters.
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut best = 0;
        let mut last_seen = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(&prev) = last_seen.get(&c) {
                if prev >= start {
                    start = prev + 1;
                }
            }
            last_seen.insert(c, i);
            best = best.max(i - start + 1);
        }

        best as i32
    }
}
fn main() {
    let s = String::from("abcabcbb");
    let res = Solution::length_of_longest_substring(s);
    println!("{}", res);

    let s = String::from("bbbbb");
    let res = Solution::length_of_longest_substring(s);
    println!("{}", res);

    let s = String::from("pwwkew");
    let res = Solution::length_of_longest_substring(s);
    println!("{}", res);

    let s = String::from("dvdf");
    let res = Solution::length_of_longest_substring(s);
    println!("{}", res);
}
