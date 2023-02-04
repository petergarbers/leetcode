use std::collections::{HashSet, HashMap};


pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut found_words: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for str in strs {
            let mut word_vec: Vec<char> = str.chars().collect();
            word_vec.sort();
            
            if found_words.contains_key(&word_vec) {
                let words_found_for_sequence = found_words.get_mut(&word_vec).unwrap();
                words_found_for_sequence.push(str);
            } else {
                found_words.insert(word_vec, vec![str]);
            }
        }
        found_words.into_values().collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn test_49() {
        let strs = vec_of_strings!["eat","tea","tan","ate","nat","bat"];
        let result = vec![vec_of_strings!["bat"], vec_of_strings!["nat", "tan"], vec_of_strings!["ate", "eat", "tea"]];
        // assert_eq!(result, Solution::group_anagrams(strs));
    }
}