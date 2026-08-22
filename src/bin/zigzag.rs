struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }
        let mut rows = vec![String::new(); num_rows as usize];
        let mut current_row = 0;
        let mut zig = false;
        for c in s.chars() {
            rows[current_row as usize].push(c);

            if current_row == 0 || current_row == num_rows - 1 {
                zig = !zig;
            }

            if zig {
                current_row += 1;
            } else {
                current_row -= 1;
            }
        }
        rows.into_iter().collect()
    }
}

fn main() {
    let s = "PAYPALISHIRING".to_string();
    let n = 3;
    let res = Solution::convert(s, n);
    println!("{}", res);
}
