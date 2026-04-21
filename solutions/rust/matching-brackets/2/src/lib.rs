pub fn brackets_are_balanced(string: &str) -> bool {
    let mut zarojelsor: Vec<char> = vec![];
    for c in string.chars() {
        if vec!['(', '[', '{'].contains(&c){
            zarojelsor.push(c);
        }
        else if vec![')', ']', '}'].contains(&c){
            if zarojelsor.len()==0{return false;}
            match &c {
                ')'=> {
                    if zarojelsor[zarojelsor.len()-1] != '('{
                        return false;
                    }
                }
                ']'=> {
                    if zarojelsor[zarojelsor.len()-1] != ']'{
                        return false;
                    }
                }
                '}'=> {
                    if zarojelsor[zarojelsor.len()-1] != '}'{
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    true
}
