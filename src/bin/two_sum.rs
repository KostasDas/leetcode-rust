use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        for (key, value) in nums.iter().enumerate() {
            let second: i32 = target - value;
            if seen.contains_key(&second) {
                let solution = seen.get(&second).unwrap();
                return vec![*solution as i32, key as i32];
            }
            seen.insert(*value, key as i32);
        }
        vec![]
    }
}

fn main() {
    let nums = vec![-3, 4, 3, 90];
    let target = 0;
    println!("{:?}", Solution::two_sum(nums, target));

    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("{:?}", Solution::two_sum(nums, target));

    let nums = vec![3, 2, 4];
    let target = 6;
    println!("{:?}", Solution::two_sum(nums, target));

    let nums = vec![3, 3];
    let target = 6;
    println!("{:?}", Solution::two_sum(nums, target));

    let nums = vec![0, 4, 3, 0];
    let target = 0;
    println!("{:?}", Solution::two_sum(nums, target));
}
