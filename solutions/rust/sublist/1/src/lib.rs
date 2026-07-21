#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.len() == second_list.len() {
        if first_list.iter().eq(second_list.iter()) {
            return Comparison::Equal
        }
        return Comparison::Unequal
    }
    if first_list.len() > second_list.len() {
        if is_superlist(first_list, second_list) {
            return Comparison::Superlist
        }
        return Comparison::Unequal
    } else if is_superlist(second_list, first_list) {
        return Comparison::Sublist
    }
    Comparison::Unequal
}

fn is_superlist(l1: &[i32], l2: &[i32]) -> bool {
    let mut it1 = l1.iter();
    let mut start1 = it1.clone();
    let mut it2 = l2.iter();
    let mut val1 = it1.next();
    let mut val2 = it2.next();
    while val1.is_some() && val2.is_some() {
        if val1.unwrap() != val2.unwrap() {
            it2 = l2.iter();
            it1 = start1.clone();
            start1.next();
        }
        val1 = it1.next();
        val2 = it2.next();
    }
    if val2.is_some() {
        return false
    }
    true
}
