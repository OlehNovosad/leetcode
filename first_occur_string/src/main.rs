struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }

        let haystack_ch: Vec<char> = haystack.chars().collect();
        let needle_ch: Vec<char> = needle.chars().collect();

        let mut index: i32 = -1;

        for i in 0..haystack_ch.len() {
            if haystack_ch[i] == needle_ch[0] && needle_ch.len() <= haystack_ch.len() - i {
                let mut matched = true;
                for j in 0..needle_ch.len() {
                    if haystack_ch[i + j] != needle_ch[j] {
                        matched = false;
                    }
                }
                if matched == true {
                    index = i as i32;
                    break;
                }
            }
        }

        index
    }
}

fn main() {
    let haystack = String::from("sadbutsad");
    let needle = String::from("sad");
    let res: i32 = Solution::str_str(haystack, needle);
    println!("{}", res);

    let haystack = String::from("hello");
    let needle = String::from("ll");
    let res: i32 = Solution::str_str(haystack, needle);
    println!("{}", res);

    let haystack = String::from("mississippi");
    let needle = String::from("issipi");
    let res: i32 = Solution::str_str(haystack, needle);
    println!("{}", res);

    let haystack = String::from("mississippi");
    let needle = String::from("issip");
    let res: i32 = Solution::str_str(haystack, needle);
    println!("{}", res);

    let haystack = String::from("abc");
    let needle = String::from("c");
    let res: i32 = Solution::str_str(haystack, needle);
    println!("{}", res);

    let haystack = String::from("abcdefgh");
    let needle = String::from("abxxxfgh");
    let res: i32 = Solution::str_str(haystack, needle);
    println!("{}", res);
}
