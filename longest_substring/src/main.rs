use std::{collections::HashMap, usize};

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut start = 0;
        let mut max_len = 0;

        for (end, ch) in s.chars().enumerate() {
            if let Some(&prev_index) = map.get(&ch)
                && prev_index + 1 > start
            {
                start = prev_index + 1;
            }
            map.insert(ch, end);
            max_len = max_len.max(end - start + 1);
        }

        max_len as i32
    }
}

fn main() {
    // abcabcbb
    let s: String = "abcabcbb".to_string();
    let res: i32 = Solution::length_of_longest_substring(s);
    println!(
        "{}",
        if res != 3 {
            "wrong".to_string()
        } else {
            res.to_string()
        }
    );

    // let s: String = "bbbbb".to_string();
    // let res: i32 = Solution::length_of_longest_substring(s);
    // println!(
    //     "{}",
    //     if res != 1 {
    //         "wrong".to_string()
    //     } else {
    //         res.to_string()
    //     }
    // );

    // let s: String = "pwwkew".to_string();
    // let res: i32 = Solution::length_of_longest_substring(s);
    // println!(
    //     "{}",
    //     if res != 3 {
    //         "wrong".to_string()
    //     } else {
    //         res.to_string()
    //     }
    // );
}
