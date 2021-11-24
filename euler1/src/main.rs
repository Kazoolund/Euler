fn main() {
    let problem1 = prob1(1000);
    println!("Problem 1 answer: {}", problem1);
}

fn prob1(amount: i64) -> i64{
    let mut i = 0;
    // let mut j = 0; Fuck the compiler
    let mut accumulator = 0;
    while i < amount{
        if i % 3 == 0{
            accumulator += i;
        }
        else if i % 5 == 0{
            accumulator += i;
        }
        i = i + 1;
    }
    accumulator
}