pub fn factors(n: u64) -> Vec<u64> {
    //todo!("This should calculate the prime factors of {n}")
    let mut number = n;
    let mut primes: Vec<u64> = Vec::new();
    let mut biggestprime = 2;
    if n == 1 {
        return vec![];
    }
    loop {
        if number == biggestprime {
            primes.push(biggestprime);
            break;
        } else if number % biggestprime == 0 {
            number /= biggestprime;
            primes.push(biggestprime);
        } else {
            biggestprime += 1;
        }
    }
    primes
}
