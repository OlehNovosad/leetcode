use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            if let Some(x) = map.get(&num) {
                map.insert(num, x + 1);
            } else {
                map.insert(num, 1);
            }
        }

        println!("{:?}", map);

        *map.iter().max_by_key(|&(k, v)| v).unwrap().0
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 3];
    let res: i32 = Solution::repeated_n_times(nums);
    println!("{:?}", res);

    let nums: Vec<i32> = vec![2, 1, 2, 5, 3, 2];
    let res: i32 = Solution::repeated_n_times(nums);
    println!("{:?}", res);

    let nums: Vec<i32> = vec![5, 1, 5, 2, 5, 3, 5, 4];
    let res: i32 = Solution::repeated_n_times(nums);
    println!("{:?}", res);
}
