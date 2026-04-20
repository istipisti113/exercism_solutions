pub fn square_of_sum(n: u32) -> u32 {
    let nums : Vec<u32> = (1..n+1).collect();
    nums.iter().sum::<u32>().pow(2)
    //todo!("square of sum of 1...{n}")
}

pub fn sum_of_squares(n: u32) -> u32 {
    let nums : Vec<u32> = (1..n+1).collect();
    nums.iter().map(|num| num.pow(2)).sum::<u32>()
    //todo!("sum of squares of 1...{n}")
}

pub fn difference(n: u32) -> u32 {
    //todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n).abs_diff(sum_of_squares(n))
}
