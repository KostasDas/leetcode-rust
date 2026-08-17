struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();

        // The best palindrome found so far, stored as a byte range [best_start, best_end).
        let mut best_start = 0;
        let mut best_end = 0;

        for i in 0..n {
            // we need to cast everything to i32 because the last step is 0 - 1 which is impossible for usize
            let mut left = i as i32;
            let mut right = i as i32;
            while left >= 0 && right < n as i32 && b[left as usize] == b[right as usize] {
                left -= 1;
                right += 1;
            }
            // Loop exits one step too far out; the palindrome is (left+1)..right.
            let (l, r) = ((left + 1) as usize, right as usize);
            if r - l > best_end - best_start {
                best_start = l;
                best_end = r;
            }

            let mut left = i as i32;
            let mut right = i as i32 + 1;
            while left >= 0 && right < n as i32 && b[left as usize] == b[right as usize] {
                left -= 1;
                right += 1;
            }
            let (l, r) = ((left + 1) as usize, right as usize);
            if r - l > best_end - best_start {
                best_start = l;
                best_end = r;
            }
        }

        s[best_start..best_end].to_string()
    }
}

fn main() {
    assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab"); // "aba" also valid
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
    assert_eq!(Solution::longest_palindrome("".to_string()), "");
    assert_eq!(Solution::longest_palindrome("aabaa".to_string()), "aabaa");
    assert_eq!(Solution::longest_palindrome("abba".to_string()), "abba");
    println!("all tests passed");
}
