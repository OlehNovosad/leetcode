struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        println!("string: {}", s);
        let chars: Vec<char> = s.chars().collect();
        let mut res: Vec<String> = Default::default();

        fn expand(chars: &Vec<char>, mut left: isize, mut right: isize, res: &mut Vec<String>) {
            while left >= 0
                && right < chars.len() as isize
                && chars[left as usize] == chars[right as usize]
            {
                res.push(
                    chars[left as usize..=right as usize]
                        .iter()
                        .copied()
                        .collect(),
                );
                left -= 1;
                right += 1;
            }
        }

        for i in 0..chars.len() {
            expand(&chars, i as isize, i as isize, &mut res);
            expand(&chars, i as isize, (i + 1) as isize, &mut res);
        }

        res.iter()
            .max_by_key(|v| v.len())
            .cloned()
            .unwrap_or_default()
    }
}

fn main() {
    let s: String = "babad".to_string();
    let res: String = Solution::longest_palindrome(s);
    println!("{}", res);

    let s: String = "cbbd".to_string();
    let res: String = Solution::longest_palindrome(s);
    println!("{}", res);
}
