/**
 * [88] Merge Sorted Array
 *
 * You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
 * Merge nums1 and nums2 into a single array sorted in non-decreasing order.
 * The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
 * Output: [1,2,2,3,5,6]
 * Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
 * The result of the merge is [<u>1</u>,<u>2</u>,2,<u>3</u>,5,6] with the underlined elements coming from nums1.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums1 = [1], m = 1, nums2 = [], n = 0
 * Output: [1]
 * Explanation: The arrays we are merging are [1] and [].
 * The result of the merge is [1].
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums1 = [0], m = 0, nums2 = [1], n = 1
 * Output: [1]
 * Explanation: The arrays we are merging are [] and [1].
 * The result of the merge is [1].
 * Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.
 * 
 *  
 * Constraints:
 * 
 * 	nums1.length == m + n
 * 	nums2.length == n
 * 	0 <= m, n <= 200
 * 	1 <= m + n <= 200
 * 	-10^9 <= nums1[i], nums2[j] <= 10^9
 * 
 *  
 * Follow up: Can you come up with an algorithm that runs in O(m + n) time?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-sorted-array/
// discuss: https://leetcode.com/problems/merge-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
  
        let mut pointer = (m + n) as usize;
        let mut m_idx = m as usize; 
        let mut n_idx = n as usize;

        nums1.insert(0, i32::MIN);
        nums2.insert(0, i32::MIN);

        while pointer > 0 {
            let m_val = nums1[m_idx];
            let n_val = nums2[n_idx];

            if m_val < n_val {
                nums1[pointer] = n_val;
                n_idx -= 1; 
            
            } else {
                nums1[pointer] = m_val;
                m_idx -= 1;
            }
            pointer -= 1;
        }
        nums1.retain(|&x| x != i32::MIN);


    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut n1 = vec![1,2,3,0,0,0];
        let mut n2 = vec![2,5,6];
        let result = vec![1,2,2,3,5,6];
        Solution::merge(&mut n1, 3, &mut n2, 3);
        assert_eq!(result, n1);
    }
}
