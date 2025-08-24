pub fn collatz(n: u64) -> Option<u64> {
    //todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    let mut number = n;
    let mut iter = 0;
    if number == 1 {
        return Some(0);
    } else if number == 0 {
        return None;
    }
    while number != 1 {
        //if number % 2 == 0 {
        //    number /= 2;
        //} else {
        //    number = number * 3 + 1;
        //}
        number = ((number % 2 == 0) as u64 * (number / 2) as u64)
            + ((number % 2 == 1) as u64 + (number * 3 + 1) as u64);
        iter += 1;
    }
    Some(iter)
}
