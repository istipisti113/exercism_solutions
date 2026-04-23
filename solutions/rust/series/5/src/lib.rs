pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut ret = Vec::<String>::new();
    if digits == ""{return vec![];}
    if len>digits.len(){return vec![];}
    for i in 0..len{
        let mut new = String::new();
        for j in i..i+len{
            new.push(digits.chars().nth(j).unwrap());
        }
        ret.push(new);
    }
    ret
}
