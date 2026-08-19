struct Solution;

/**
* A cinema has n rows of seats, numbered from 1 to n. Each row has 10 seats, numbered from 1 to 10.

You are given a 2D integer array reservedSeats, where reservedSeats[i] = [rowi, seati] means that seat seati in row rowi is already reserved.

A four-person group must be assigned to four seats in the same row. The group can be seated in one of the following seat blocks:
    seats 2, 3, 4, 5
    seats 4, 5, 6, 7
    seats 6, 7, 8, 9
A block can be used only if none of its seats are reserved. Each seat can be assigned to at most one group.
Return an integer denoting the maximum number of four-person groups that can be assigned.
*/
impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut cons_seat_count = 0;
        let allow_list = vec![2, 4, 6];
        for i in 1..n + 1 {
            let mut to_check = vec![];
            let reserved_row_seats: Vec<i32> = reserved_seats
                .iter()
                .filter(|seat| seat[0] == i)
                .map(|seat| seat[1])
                .collect();
            for j in &allow_list {
                if !reserved_row_seats.contains(j) && !reserved_row_seats.contains(&(j + 2)) {
                    to_check.push(*j);
                }
            }

            let mut skip_next = false;
            for start in &to_check {
                if skip_next {
                    skip_next = false;
                    continue;
                }
                let mut can_allocate = true;
                for j in start + 1..start + 4 {
                    if reserved_row_seats.contains(&j) {
                        can_allocate = false;
                    }
                }
                if can_allocate {
                    skip_next = true;
                    cons_seat_count += 1;
                }
            }
        }
        cons_seat_count
    }
}

fn main() {
    let rows = 3;
    let reserved = vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 8],
        vec![2, 6],
        vec![3, 1],
        vec![3, 10],
    ];
    let result = Solution::max_number_of_families(rows, reserved);
    println!("{:?}", result);

    let rows = 2;
    let reserved = vec![
        vec![2, 9],
        vec![2, 7],
        vec![2, 3],
        vec![1, 4],
        vec![2, 8],
        vec![1, 7],
        vec![2, 10],
        vec![1, 6],
        vec![2, 2],
        vec![1, 5],
    ];
    let result = Solution::max_number_of_families(rows, reserved);
    println!("{:?}", result);
}
