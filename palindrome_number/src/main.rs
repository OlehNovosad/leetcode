struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str: String = x.to_string();
        let x_str_rev: String = x_str.chars().rev().collect();

        if x_str != x_str_rev {
            return false;
        }

        true
    }
}

fn main() {
    let x: i32 = 121;
    let res: bool = Solution::is_palindrome(x);
    println!("{}", res);

    let x: i32 = -121;
    let res: bool = Solution::is_palindrome(x);
    println!("{}", res);

    let x: i32 = 10;
    let res: bool = Solution::is_palindrome(x);
    println!("{}", res);
}
