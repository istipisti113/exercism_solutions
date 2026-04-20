pub fn nth(n: u32) -> u32 {
    //todo!("What is the 0-indexed {n}th prime number?")
    let mut primes = vec![2];
    let mut primecounter = 1;
    let mut num = 2;
    while primecounter<n {
        num +=1;
        let mut isprime = true;
        for p in &primes {
            if num%p == 0{
                isprime=false;
            }
        }
        if isprime{ primes.push(num); primecounter+=1;}
    }
    primes[primes.len()-1].clone()
}
