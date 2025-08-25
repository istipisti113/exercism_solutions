pub fn is_valid(original: &str) -> bool {
    if original
        .chars()
        .into_iter()
        .filter(|x| x.is_numeric())
        .count()
        == 1
    {
        //one numerical value is invalid, but does not remove spaces
        return false;
    }
    if original
        .chars()
        .into_iter()
        .any(|x| !x.is_digit(10) && x != ' ')
    //if it has anything other than numbers and spaces it is incorrect
    {
        return false;
    }
    let mut code: Vec<u32> = original
        .chars()
        .filter(|x| x != &' ')
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
}
