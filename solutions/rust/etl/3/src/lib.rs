use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut output = BTreeMap::new();
    for num in h.keys(){
        if let Some(letters) = h.get(num){
            for letter in letters {
                output.insert(letter.to_ascii_lowercase(), *num);
            }
        }
    }
    output
}
