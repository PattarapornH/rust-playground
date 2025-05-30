// https://leetcode.com/problems/two-sum/

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    ans.push(i as i32);
                    ans.push(j as i32);
                    return ans;
                }
            }
        }
        return ans;
    }
}
