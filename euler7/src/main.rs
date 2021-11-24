use primes::PrimeSet;

fn main() {
    let answer: u64 = prob7();

    println!("Answer: {}", answer);
}

fn prob7() -> u64 {
    let pset = PrimeSet::new().get(10000);

    pset
}

fn prob7komma5(){
    let mut pset = PrimeSet::new();
    let ps = pset.iter();
    let mut i = 0;
    for prime in ps{
        if prime > limit{
            println!("{}", prime);
            i = prime;
            return i;
        }
    }
    i
}