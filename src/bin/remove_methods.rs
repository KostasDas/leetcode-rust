use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj: Vec<Vec<i32>> = vec![Vec::new(); n as usize];

        // this builds us who invokes who
        for pair in &invocations {
            let caller = pair[0] as usize;
            let callee = pair[1];
            adj[caller].push(callee);
        }

        let mut queue = VecDeque::new();
        queue.push_back(k);
        let mut suspicious: HashSet<i32> = HashSet::new();
        suspicious.insert(k);
        // dfs
        while let Some(m) = queue.pop_front() {
            for &callee in &adj[m as usize] {
                if !suspicious.contains(&callee) {
                    suspicious.insert(callee);
                    queue.push_back(callee);
                }
            }
        }
        let contaminated = invocations.iter().any(|pair| {
            let caller = pair[0];
            let callee = pair[1];
            suspicious.contains(&callee) && !suspicious.contains(&caller)
        });

        if contaminated {
            (0..n).collect()
        } else {
            (0..n).filter(|m| !suspicious.contains(m)).collect()
        }
    }
}

fn main() {
    let n = 4;
    let k = 1;
    let invocations = vec![vec![1, 2], vec![0, 1], vec![3, 2]];
    // should be all methods
    let res = Solution::remaining_methods(n, k, invocations);
    print!("{:?}", res);

    let n = 5;
    let k = 0;
    let invocations = vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]];
    // should be [3,4]
    let res = Solution::remaining_methods(n, k, invocations);
    print!("{:?}", res);

    let n = 2;
    let k = 0;
    let invocations = vec![];
    // should be [1]
    let res = Solution::remaining_methods(n, k, invocations);
    print!("{:?}", res);
}
