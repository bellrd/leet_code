struct Solution();
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = String::from("");
        // find smallest index
        let mut smallest_string_index = 0;
        for (index, value) in strs.iter().enumerate() {
            if value.len() < strs[smallest_string_index].len() {
                smallest_string_index = index;
            }
        }
        let temp: Vec<char> = strs[smallest_string_index].chars().collect();
        let strs: Vec<Vec<char>> = strs
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        for (index, char) in temp.iter().enumerate() {
            if strs.iter().all(|word| char == &word[index]) {
                prefix.push(*char);
            } else {
                break;
            }
        }
        return prefix;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn it_works() {
        let v: Vec<String> = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        let s = Solution::longest_common_prefix(v);
        assert_eq!(s, String::from("fl"));
    }
}

