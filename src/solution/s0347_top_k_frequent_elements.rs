use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        for i in nums {
            *seen.entry(i).or_insert(0) += 1;
        }

        let mut moo: Vec<(i32, i32)> = seen.into_iter().collect();
        moo.sort_by(|a, b| b.1.cmp(&a.1));

        moo.iter().take(k as usize).map(|x| x.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moo() {
        let result = 2 + 2;
        let nums = vec![1,1,1,2,2,3];
        let n = 2;
        let result = vec![1, 2];
        assert_eq!(Solution::top_k_frequent(nums, n), result);
    }
}