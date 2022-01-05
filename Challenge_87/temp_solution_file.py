def solution():
    /// Given an integer K greater than 2, find base B such as K can be written in the form of only one or more 1s in base B.
    /// Output positive integer B in which K can be written as one or more 1s in base B.
    /// If multiple Bs satisfy, output B with the most 1s after writing K in base B.
    pub fn solve(k: u64) -> u64 {
        for b in (2..k).filter(|b| k % b == 1) {
            let mut j = k;
            for i in 0..64 {
                match j.checked_sub(b.pow(i)) {
                    Some(j2) => {
                        j = j2;
                    }
                    None => {
                        break;
                    }
                }
                if j == 0 {
                    return b;
                }
            }
        }
        unreachable!();
    }
