pub fn is_valid(codee: &str) -> bool {
    if codee.len() == 1 {
        return false;
    }
    if codee
        .chars()
        .into_iter()
        .any(|x| !x.is_digit(10) && x != ' ')
    {
        return false;
    }
    let mut code: Vec<u32> = codee
        .chars()
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
