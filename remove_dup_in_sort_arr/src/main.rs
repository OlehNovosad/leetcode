use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut nums_set = HashSet::new();
        nums.retain(|x| nums_set.insert(*x));

        nums.len() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
