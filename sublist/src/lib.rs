use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    match () {
        () if is_equal(first_list, second_list) => Comparison::Equal,
        () if is_sublist(first_list, second_list) => Comparison::Sublist,
        () if is_sublist(second_list, first_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

fn is_sublist<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|window| window == first_list)
}

fn is_equal<T: PartialEq + Debug>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.len() == second_list.len()
        && first_list
            .iter()
            .zip(second_list)
            .all(|(item_a, item_b)| item_a == item_b)
}
