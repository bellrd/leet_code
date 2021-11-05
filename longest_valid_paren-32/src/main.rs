struct Solution();
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::<char>::new();
        let mut result = 0;
        let mut s: Vec<char> = s.chars().collect();
        for p in s {
            if p == '(' {
                stack.push('(');
                continue;
            }
            if let Some('(') = stack.last() {
                stack.pop();
                result += 1;
            }
        }

        return result * 2;
    }
}
fn main() {
    println!("Hello, world!");
}

#[test]
fn it_works() {
    let s = Solution();
    let t = "()(()".to_owned();
    assert_eq!(2, Solution::longest_valid_parentheses(t));
}
