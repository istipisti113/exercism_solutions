pub fn is_valid(codee: &str) -> bool {
    let mut code: Vec<char> = codee.chars().rev().collect();
    for i in (1..code.len()).step_by(2) {
        let mut asdf = (code[i].to_digit(10).unwrap()) * 2;
        if asdf > 9 {
            asdf -= 9;
        }
        code[i] = char::from_u32(asdf).unwrap();
    }
    code.iter()
        .map(|x| x.to_digit(10).unwrap())
        .into_iter()
        .sum::<u32>()
        % 10
        == 0
    //todo!("Is the Luhn checksum for code valid?");
}
