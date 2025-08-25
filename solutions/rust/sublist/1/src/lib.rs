#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    //todo!("Determine if the {first_list:?} is equal to, sublist of, superlist of or unequal to {second_list:?}.");
    if first_list.len() == 0 && second_list.len() == 0 {
        return Comparison::Equal;
    } else if first_list.len() == 0 {
        return Comparison::Sublist;
    } else if second_list.len() == 0 {
        return Comparison::Superlist;
    } else {
        if first_list.len() >= second_list.len() {
            let mut matching = true;
            for i in 0..first_list.len() {
                if first_list[i] != second_list[i] {
                    matching = false;
                    break;
                }
            }
            if matching {
                if first_list.len() > second_list.len() {
                    return Comparison::Superlist;
                } else {
                    return Comparison::Equal;
                }
            } else {
                return Comparison::Unequal;
            }
        } else {
            let mut matching = true;
            for i in 0..second_list.len() {
                if second_list[i] != first_list[i] {
                    return Comparison::Unequal;
                }
            }
            return Comparison::Sublist;
        }
    }
}
