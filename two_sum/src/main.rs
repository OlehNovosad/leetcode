use std::{collections::HashMap, vec};

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create HashMap
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            // Calculate diff between target and num
            let calc = target - num;

            // if calc is present in hashmap
            if let Some(&j) = map.get(&calc) {
                return vec![j, i as i32];
            }

            // if we didn't find calc in hashmap insert key: num and i: value
            map.insert(num, i as i32);
        }

        // if we didn't find two var return empty vector
        vec![]
    }
}

fn main() {
    let num: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result = Solution::two_sum(num, target);
    println!("Result: {:?}", result);

    let num = vec![3, 2, 4];
    let target = 6;
    let result = Solution::two_sum(num, target);
    println!("Result: {:?}", result);

    let num = vec![3, 3];
    let target = 6;
    let result = Solution::two_sum(num, target);
    println!("Result: {:?}", result);
}
