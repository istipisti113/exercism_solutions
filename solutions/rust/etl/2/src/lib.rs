use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let output = BTreeMap::new();
    for num in h.keys(){
        for letters in h.get(num){
            for letter in letters {
                println!("{}", letter);
            }
        }
    }
    output
}
