struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp;

        let (mut curr_sum, mut global_sum) = (0, i32::MIN);

        for &num in nums.iter() {
            curr_sum += num;
            global_sum = cmp::max(curr_sum, global_sum);

            if curr_sum < 0 {
                curr_sum = 0;
            }
        }

        global_sum
    }
}

fn main() {
    let nums = vec![-2, -3, 4, -1, -2, 1, 5, -3];
    let max_sum = Solution::max_sub_array(nums);

    assert_eq!(max_sum, 7);
    println!("The maximum subarray sum is: {}", max_sum);
}



