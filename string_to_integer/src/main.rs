struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s_chars: Vec<char> = s.trim().chars().collect();
        let mut res_str: String = Default::default();
        let mut res: i32 = 0_i32;

        if s_chars.is_empty() {
            return res;
        }

        for ch in 0..s_chars.len() {
            if ch == 0 {
                if s_chars[0] == '-' || s_chars[0] == '+' {
                    // check if next character exists and is digit
                    if s_chars.len() == 1 || !s_chars[1].is_ascii_digit() {
                        return 0;
                    }
                    res_str.push(s_chars[0]);
                    continue;
                } else if !s_chars[0].is_ascii_digit() {
                    return 0;
                }
            }

            if s_chars[ch].is_ascii_digit() {
                res_str.push(s_chars[ch]);
            } else {
                break;
            }
        }

        if !res_str.is_empty() {
            let mut res: i32 = 0;
            let mut sign = 1;
            let mut start = 0;

            // handle sign
            if res_str.starts_with('-') {
                sign = -1;
                start = 1;
            } else if res_str.starts_with('+') {
                start = 1;
            }

            for ch in res_str.chars().skip(start) {
                let digit = ch as i32 - '0' as i32;

                // check overflow
                if res > (i32::MAX - digit) / 10 {
                    return if sign == 1 { i32::MAX } else { i32::MIN };
                }

                res = res * 10 + digit;
            }

            return res * sign;
        }

        res
    }
}

fn main() {
    // let s: String = "42".to_string();
    // let res: i32 = Solution::my_atoi(s);
    // println!("{}", res);

    // let s: String = "-042".to_string();
    // let res: i32 = Solution::my_atoi(s);
    // println!("{}", res);

    // let s: String = "1337c0d3".to_string();
    // let res: i32 = Solution::my_atoi(s);
    // println!("{}", res);

    // let s: String = "0-1".to_string();
    // let res: i32 = Solution::my_atoi(s);
    // println!("{}", res);

    // let s: String = "words and 987".to_string();
    // let res: i32 = Solution::my_atoi(s);
    // println!("{}", res);

    // let s: String = "-91283472332".to_string();
    // let res: i32 = Solution::my_atoi(s);
    // println!("{}", res);

    // let s: String = "   -042".to_string();
    // let res: i32 = Solution::my_atoi(s);
    // println!("{}", res);

    // let s: String = "+-12".to_string();
    // let res: i32 = Solution::my_atoi(s);
    // println!("{}", res);

    let s: String = "20000000000000000000".to_string();
    let res: i32 = Solution::my_atoi(s);
    println!("{}", res);
}
