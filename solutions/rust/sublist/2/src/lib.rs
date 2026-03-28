#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_equal(first_list:&[i32], second_list:&[i32]) -> bool{
     
   first_list == second_list

}


pub fn contains_sublist(small: &[i32], large: &[i32]) -> bool {
    let small_len = small.len();
    let large_len = large.len();

    if small_len > large_len {
        return false;
    }

    for i in 0..=large_len - small_len {
        if small == &large[i..i + small_len] {
            return true;
        }
    }

    false
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
 

    if first_list == second_list {
        return Comparison::Equal;
    } else if contains_sublist(first_list, second_list) {
        return Comparison::Sublist;
    } else if contains_sublist(second_list, first_list){
        return Comparison::Superlist;
    } 

    return Comparison::Unequal;

     
}
