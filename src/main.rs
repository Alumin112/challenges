use std::io;

mod challenge87;
fn main() {
    for input in input_i32() {
        println!("{}", challenge87::solve(input));
    }
}

fn input_i32() -> Vec<i32> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    let inputs: i32 = input
        .trim()
        .parse().unwrap();

    return (0..inputs)
        .map(|_| {
            let mut input = String::new();

            io::stdin().read_line(&mut input)
                .expect("Failed to read line");

            input.trim().parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();
}
