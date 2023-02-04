use std::collections::{HashSet, HashMap};


pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // let mut word_sets: Vec<HashSet<char>> = Vec::new();
        let found_words: HashMap<String, Vec<String>> = HashMap::new();
        for str in strs {
            let word_set: HashSet<char> = str.chars().collect();

            // if found_words.contains_key(word_set) {

            // }

            // found_words.insert(str.to_string(), vec![str.to_string()]);
        }

        // println!("{:?}", word_sets);

        vec![vec!["woof".to_string()]]
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn test_49() {
        let woof: HashSet<char> = "woof".chars().collect();
        let moo: HashMap<HashSet<char>, Vec<String>> = HashMap::new();

        let unsorted: String = "Yourmom".to_string();
        let mut chars: Vec<char> = unsorted.chars().collect(); 
        // chars.sort_by(|a,b| b.cmp(a))
        println!("{:?}",chars.sort_by(|a, b| b.cmp(a)));

        let strs = vec_of_strings!["eat","tea","tan","ate","nat","bat"];
        // let result = vec![vec_of_strings!["bat"], vec_of_strings!["nat", "tan"], vec_of_strings!["ate", "eat", "tea"]];
        Solution::group_anagrams(strs);
        // assert_eq!(result, Solution::group_anagrams(strs));
    }
}