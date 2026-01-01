struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = digits.clone();
        let mut carry = 1;

        for i in (0..res.len()).rev() {
            let sum = res[i] + carry;
            res[i] = sum % 10;
            carry = sum / 10;

            if carry == 0 {
                break;
            }
        }

        if carry > 0 {
            res.insert(0, carry);
        }

        res
    }
}

fn main() {
    let digits: Vec<i32> = vec![1, 2, 3];
    let res: Vec<i32> = Solution::plus_one(digits);
    println!("{:?}", res);

    // let digits: Vec<i32> = vec![4, 3, 2, 1];
    // let res: Vec<i32> = Solution::plus_one(digits);
    // println!("{:?}", res);

    // let digits: Vec<i32> = vec![9];
    // let res: Vec<i32> = Solution::plus_one(digits);
    // println!("{:?}", res);

    // let digits: Vec<i32> = vec![9, 9];
    // let res: Vec<i32> = Solution::plus_one(digits);
    // println!("{:?}", res);
}
