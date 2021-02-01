struct Solution;

#[allow(non_snake_case)]
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut count = 0;

        let mut n = n;
        while n > 0 {
            n = n & (n - 1);
            count += 1;
        }

        count
    }
}

#[test]
fn hamming_weight() {
    let cases = vec![
        (00000000000000000000000000001011, 3),
        (00000000000000000000000000000001, 1),
        (00000000000000000000000010000000, 1),
    ];

    for case in cases {
        assert_eq!(Solution::hammingWeight(case.0), case.1)
    }
}

pub fn main() {
    Solution::hammingWeight(00000000000000000000000010000000);
}
