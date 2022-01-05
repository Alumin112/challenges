use std::io;

mod challenge87 {
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
}

fn main() {
    for input in input_i32() {
        println!("{}", challenge87::solve(input));
    }
}

fn input_i32() -> Vec<u64> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    let inputs: u64 = input
        .trim()
        .parse().unwrap();

    return (0..inputs)
        .map(|_| {
            let mut input = String::new();

            io::stdin().read_line(&mut input)
                .expect("Failed to read line");

            input.trim().parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>();
}
