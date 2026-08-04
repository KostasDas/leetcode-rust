# LeetCode in Rust

My solutions to some LeetCode problems, written in Rust.
Each problem is a standalone binary under `src/bin/`, runnable with:

```bash
cargo run --bin <file_name>
```

I tried not to use AI since this is a learning process, but I did ask for hints when stuck for a long time. 

## Problems

| # | Problem | File | Difficulty | Pattern | Status |
|---|---------|------|-----------|---------|--------|
| 1 | [Two Sum](https://leetcode.com/problems/two-sum/description/) | [`two_sum.rs`](src/bin/two_sum.rs) | Easy | Hash map (one-pass complement lookup) | ✅ |
| 2 | [Add Two Numbers](https://leetcode.com/problems/add-two-numbers/description/) | [`add_two_numbers.rs`](src/bin/add_two_numbers.rs) | Medium | Linked list, dummy head + tail pointer, carry propagation | ✅ |
| — | [Find Missing Elements](https://leetcode.com/problems/find-missing-elements/description) | [`find_missing_elements.rs`](src/bin/find_missing_elements.rs) | Easy | Sorting / range scan | ✅ |
