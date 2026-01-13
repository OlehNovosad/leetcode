struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        let mut char_found = false;

        for i in s.chars().rev() {
            if i.is_alphabetic() {
                char_found = true;
                count += 1;
            }

            if !i.is_alphabetic() && char_found {
                break;
            }
        }

        count
    }
}

fn main() {
    let s: String = String::from("Hello World");
    let res = Solution::length_of_last_word(s);
    println!("{}", res);

    let s: String = String::from("   fly me   to   the moon  ");
    let res = Solution::length_of_last_word(s);
    println!("{}", res);

    let s: String = String::from("luffy is still joyboy");
    let res = Solution::length_of_last_word(s);
    println!("{}", res);
}
