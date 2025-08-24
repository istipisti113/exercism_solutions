pub fn square(s: u32) -> u64 {
    let mut squares = vec![];
    let mut lastnumber = 1;
    for _ in 0..=s {
        squares.push(lastnumber);
        lastnumber *= 2;
    }
    squares.iter().sum()
}

pub fn total() -> u64 {
    todo!();
}
