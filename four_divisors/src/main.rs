use core::num;

struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut divisor_vec: Vec<Vec<i32>> = Vec::new();
        let mut res: i32 = 0;

        for num in nums {
            let mut tmp: Vec<i32> = Vec::new();
            for i in 1..=num.isqrt() {
                if num % i == 0 {
                    tmp.push(i);
                    if !tmp.contains(&(num / i)) {
                        tmp.push(num / i);
                    }
                }
            }
            divisor_vec.push(tmp);
        }

        for divisor in divisor_vec {
            if divisor.len() == 4 {
                res += divisor.iter().sum::<i32>();
            }
        }

        res
    }
}

fn main() {
    let nums: Vec<i32> = vec![21, 4, 7];
    let res: i32 = Solution::sum_four_divisors(nums);
    println!("{}", res);

    // let nums: Vec<i32> = vec![21, 21];
    // let res: i32 = Solution::sum_four_divisors(nums);
    // println!("{}", res);

    // let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    // let res: i32 = Solution::sum_four_divisors(nums);
    // println!("{}", res);
}
