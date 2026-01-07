struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut abs_sum: i64 = 0;
        let mut neg_count: i64 = 0;
        let mut min_var: i64 = i64::MAX;

        for v in matrix {
            for i in v {
                abs_sum += i.abs() as i64;
                if i < 0 {
                    neg_count += 1;
                }

                min_var = min_var.min(i.abs() as i64);
            }
        }

        if neg_count % 2 != 0 {
            abs_sum = (abs_sum as i128 - (2 * min_var as i128)) as i64;
        }

        abs_sum
    }
}

fn main() {
    let matrix: Vec<Vec<i32>> = vec![vec![1, -1], vec![-1, 1]];
    let res: i64 = Solution::max_matrix_sum(matrix);
    println!("{}", res);

    let matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
    let res: i64 = Solution::max_matrix_sum(matrix);
    println!("{}", res);
}
