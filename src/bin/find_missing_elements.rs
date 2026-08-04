struct Solution;

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut mut_nums = nums;
        mut_nums.sort();
        let mut result: Vec<i32> = vec![];
        let mut expected = mut_nums[0];
        for value in mut_nums.iter() {
            let v = *value;
            if v == expected {
                expected += 1;
                continue;
            }
            while expected < v {
                result.push(expected);
                expected += 1;
            }
            expected += 1;
        }
        result
    }
}

fn main() {
    let nums = vec![1, 4, 2, 5];
    let result = Solution::find_missing_elements(nums);
    print!("{:?}", result);

    let nums = vec![7, 8, 6, 9];
    let result = Solution::find_missing_elements(nums);
    print!("{:?}", result);

    let nums = vec![5, 1];
    let result = Solution::find_missing_elements(nums);
    print!("{:?}", result);
}
