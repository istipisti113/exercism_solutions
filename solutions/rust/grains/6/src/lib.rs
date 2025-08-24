pub fn square(s: u32) -> u64 {
    let mut lastnumber = 1;
    for _ in 0..s - 1 {
        lastnumber *= 2;
    }
    lastnumber
}

pub fn total() -> u64 {
    let mut squares = vec![];
    let mut lastnumber = 1;
    for _ in 0..64 {
        squares.push(lastnumber);
        lastnumber *= 2;
    }
    squares.iter().sum()
}
