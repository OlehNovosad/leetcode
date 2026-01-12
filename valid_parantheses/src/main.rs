use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let parantheses_map: HashMap<char, char> =
            HashMap::from([('}', '{'), (')', '('), (']', '[')]);
        let mut stack: Vec<char> = Default::default();

        for ch in s.chars() {
            if let Some(value) = parantheses_map.get(&ch) {
                if stack.pop() != Some(*value) {
                    return false;
                }
            } else {
                stack.push(ch);
            }
        }

        stack.is_empty()
    }
}

fn main() {
    let s: &str = "()";
    let res: bool = Solution::is_valid(s.to_string());
    println!("{}", res);
}
