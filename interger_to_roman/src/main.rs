use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut value_symbols: HashMap<i32, &str> = HashMap::from([
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ]);

        let mut mut_num: i32 = num;
        let mut res: String = Default::default();
        let mut keys: Vec<i32> = value_symbols.keys().cloned().collect();
        keys.sort_by(|a, b| b.cmp(a));

        for value in keys {
            while mut_num >= value {
                res += value_symbols[&value];
                mut_num -= value;
            }
        }

        res
    }
}

fn main() {
    let num: i32 = 3749;
    let res: String = Solution::int_to_roman(num);
    println!("{}", res);
}
