pub fn is_valid(codee: &str) -> bool {
    let mut code: Vec<u32> = codee
        .chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap())
        .rev()
        .collect();
    for i in (1..code.len()).step_by(2) {
        let mut asdf = code[i] * 2;
        if asdf > 9 {
            asdf -= 9;
        }
        code[i] = asdf;
    }
    code.iter().into_iter().sum::<u32>() % 10 == 0
    //todo!("Is the Luhn checksum for code valid?");
}
