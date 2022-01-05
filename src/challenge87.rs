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
