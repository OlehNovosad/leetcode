struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut res = 0i32;

        for j in 0..nums.len() {
            for i in 1..nums.len() {
                if nums[i - 1] == val {
                    nums.swap(i - 1, i);
                }
            }
        }

        for i in nums {
            if *i != val {
                res += 1;
            }
        }

        res
    }
}

fn main() {
    let mut nums: Vec<i32> = vec![3, 2, 2, 3];
    let val: i32 = 3;
    let res = Solution::remove_element(&mut nums, val);
    println!("{}", res);

    let mut nums: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val: i32 = 2;
    let res = Solution::remove_element(&mut nums, val);
    println!("{}", res);
}
