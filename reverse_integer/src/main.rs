struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut i32_str: String = x.abs().to_string();
        i32_str = i32_str.chars().rev().collect();

        let res: i32 = i32_str.parse().unwrap_or_default();

        if x >= 0 { res } else { -res }
    }
}

fn main() {
    let x: i32 = 123;
    let res: i32 = Solution::reverse(x);
    println!("{}", res);

    let x: i32 = -123;
    let res: i32 = Solution::reverse(x);
    println!("{}", res);

    let x: i32 = 120;
    let res: i32 = Solution::reverse(x);
    println!("{}", res);
}
