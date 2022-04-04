// https://leetcode-cn.com/problems/shu-zu-zhong-de-ni-xu-dui-lcof/
struct Solution;
impl Solution {
    pub fn mergeSort(nums: &mut Vec<i32>, tmp: &mut Vec<i32>, l: i32, r: i32) -> i32 {
        if r - l <= 1 { return 0; }
        let mut count = 0;
        let mut mid = l + (r - l) / 2;
        count += Self::mergeSort(nums, tmp, l, mid) + Self::mergeSort(nums, tmp, mid, r);
        let (mut i, mut j, mut k) = (l, mid, l);
        while i < mid || j < r {
            if i == mid {
                tmp[k as usize] = nums[j as usize];
                j += 1;
            } else if j == r {
                tmp[k as usize] = nums[i as usize];
                i += 1;
            } else if nums[i as usize] <= nums[j as usize] {
                tmp[k as usize] = nums[i as usize];
                i += 1;
            } else {
                tmp[k as usize] = nums[j as usize];
                count += mid - i;
                j += 1;
            }
            k += 1;
        }
        for i in l..r {
            let i = i as usize;
            nums[i] = tmp[i];
        }
        return count;
    }
    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut tmp = vec![0; n];
        return Self::mergeSort(&mut nums, &mut tmp, 0, n as i32);
    }
}