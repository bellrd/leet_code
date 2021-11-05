struct Solution();
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut sign = 1;
        if x < 0 {
            sign = -1;
            x = -x;
        }
        let mut ans = String::new();
        while x != 0 {
            let r = x % 10;
            x = x / 10;
            // ans.push(char::from_digit(r as u32, 10).unwrap());
            let c = r.to_string().chars().next().unwrap();
            ans.push(c);
        }
        return ans.parse::<i32>().unwrap_or(0) * sign;
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let result = Solution::reverse(-123);
        assert_eq!(result, -321);
    }
}
