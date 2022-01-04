pub fn solve(k: i32) -> u32 {
    // Given an integer K greater than 2, find base B such as K can be written in the form of only one or more 1s in base B.
    // Output:
    // For each test case:
    // Output positive integer B in which K can be written as one or more 1s in base B.
    // If multiple Bs satisfy, output B with the most 1s after writing K in base B.
    for b in 2.. {
        let mut k_in_b = k;
        while k_in_b > 0 && k_in_b % b == 1 {
            k_in_b /= b;
        }
        if k_in_b == 0 {
            return b as u32;
        }
    }
    return 0;
}
