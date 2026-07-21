#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (len1, len2) if len1 > len2 => {
            if first_list.windows(len2).any(|sublist| sublist == second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        },
        (len1, len2) if len2 > len1 => {
            if second_list.windows(len1).any(|sublist| sublist == first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        },
        (_, _) => if first_list == second_list { Comparison::Equal } else { Comparison::Unequal }
    }
}
