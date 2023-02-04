use std::{collections::HashMap, vec};

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut cache: HashMap<i32, i32> = HashMap::new();

        for (i, x) in nums.iter().enumerate() {
            match cache.get(&(target - *x)) {
                Some(&y) => return vec![y, i as i32],
                None => cache.insert(*x, i as i32),
                
            };
        }   

        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut n1 = vec![2,7,11,15];
        let result = vec![0,1];
        assert_eq!(result, Solution::two_sum(n1, 9));
    }
}