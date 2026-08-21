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
| 3 | [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/description/) | [`longest_substring.rs`](src/bin/longest_substring.rs) | Medium | Sliding window + hash map (last-seen positions) | ✅ |
| 7 | [Reverse Integer](https://leetcode.com/problems/reverse-integer/description/) | [`reverse_integer.rs`](src/bin/reverse_integer.rs) | Medium | Math, overflow-safe arithmetic (`checked_*`) | ✅ |
| 3310 | [Remove Methods From Project](https://leetcode.com/problems/remove-methods-from-project/description) | [`remove_methods.rs`](src/bin/remove_methods.rs) | Medium | Graph traversal (BFS on adjacency list), reachability | ✅ |
| 3731 | [Find Missing Elements](https://leetcode.com/problems/find-missing-elements/description) | [`find_missing_elements.rs`](src/bin/find_missing_elements.rs) | Easy | Sorting / range scan | ✅
| 5 | [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/description/) | [`longest_palidrome.rs`](src/bin/longest_palindrome.rs) | Medium | Sorting / range scan | ✅ |
| 1386 | [Cinema Seat Allocation](https://leetcode.com/problems/cinema-seat-allocation/description/) | [`cinema_seat_allication.rs`](src/bin/cinema_seat_allication.rs) | Medium | Array iteration / Linear search | ☑️ (Does not pass performance tests) |
| 11 | [Container With Most Water](https://leetcode.com/problems/container-with-most-water/description/) | [`container.rs`](src/bin/container.rs) | Medium | Array iteration | ✅ |
