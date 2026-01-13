use core::num;

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target <= nums[0] {
            return 0i32;
        }

        for i in 0..nums.len() - 1 {
            if target > nums[i] && target <= nums[i + 1] {
                return (i + 1) as i32;
            }
        }

        nums.len() as i32
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 3, 5, 6];
    let target: i32 = 5;
    let res = Solution::search_insert(nums, target);
    println!("{}", res);

    let nums: Vec<i32> = vec![1, 3, 5, 6];
    let target: i32 = 2;
    let res = Solution::search_insert(nums, target);
    println!("{}", res);

    let nums: Vec<i32> = vec![1, 3, 5, 6];
    let target: i32 = 7;
    let res = Solution::search_insert(nums, target);
    println!("{}", res);
}
