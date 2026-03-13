#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // A and B are equal.
    if is_equal(first_list, second_list) {
        return Comparison::Equal
    }

    // A is a sublist of B.
    if is_sublist(first_list, second_list) {
        return Comparison::Sublist
    }
    

    // A is a superlist of B.
    if is_sublist(second_list, first_list) {
        return Comparison::Superlist
    }

    Comparison::Unequal
}

fn is_equal(first: &[i32], second: &[i32]) -> bool {
    first.eq(second)
}

fn is_sublist(first: &[i32], second: &[i32]) -> bool {
    let mut result = false;
    for i in 0..second.len() {
        if second[i..].starts_with(first) {
            result = true
        }
    }
    result
}

