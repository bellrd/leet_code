use std::collections::HashMap;
struct Solution();
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(2);
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for (index, value) in nums.iter().enumerate() {
            let required = target - value;
            if let Some(x) = hm.get(&required) {
                result.push(x.to_owned() as i32);
                result.push(index.to_owned() as i32);
                return result;
            }
            hm.insert(value.to_owned(), index.to_owned());
        }
        return result;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let result = Solution::two_sum([3,2,4].to_vec(), 6);
        assert_eq!(result, [1,2].to_vec());
    }
}
