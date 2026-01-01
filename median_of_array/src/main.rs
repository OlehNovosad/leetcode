struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merge: Vec<i32> = Default::default();

        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] <= nums2[j] {
                merge.push(nums1[i]);
                i += 1;
            } else {
                merge.push(nums2[j]);
                j += 1;
            }
        }

        while i < nums1.len() {
            merge.push(nums1[i]);
            i += 1;
        }

        while j < nums2.len() {
            merge.push(nums2[j]);
            j += 1;
        }

        if merge.len() % 2 != 0 {
            merge[merge.len() / 2] as f64
        } else {
            ((merge[merge.len() / 2] as f64 + merge[(merge.len() / 2) - 1] as f64) / 2.0)
        }
    }
}

fn main() {
    let nums1: Vec<i32> = vec![1, 3];
    let nums2: Vec<i32> = vec![2];
    let res: f64 = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("{}", res);

    let nums1: Vec<i32> = vec![1, 2];
    let nums2: Vec<i32> = vec![3, 4];
    let res: f64 = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("{}", res);
}
