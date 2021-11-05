use std::collections::HashSet;
struct Solution();
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            let mut hs = HashSet::new();
            for j in i..chars.len() {
                if hs.contains(&chars[j]) {
                    if hs.len() > longest {
                        longest = hs.len();
                    }
                    break;
                } else {
                    hs.insert(chars[j].to_owned());
                    if hs.len() > longest {
                        longest = hs.len();
                    }
                }
            }
        }
        return longest as i32;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let result = Solution::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(result, 3)
    }
}
