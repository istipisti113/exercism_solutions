pub fn nth(n: u32) -> u32 {
    //todo!("What is the 0-indexed {n}th prime number?")
    let mut primes = vec![2];
    let mut num = 3;
    while primes.len()!=n as usize {
        let mut isprime = true;
        for p in &primes {
            if *p as f32 > (num as f32).sqrt() {
                break;
            }
            if num%p == 0{
                isprime=false;
                break;
            }
        }
        if isprime{ primes.push(num)}
        num +=2;
    }
    primes[primes.len()-1].clone()
}
