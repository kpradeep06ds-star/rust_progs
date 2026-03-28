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

pub fn is_superlist(first_list:&[i32], second_list:&[i32]) -> bool{
    let  lenv = second_list.len() as usize; // small
    let  lenu = first_list.len() as usize; // large
    if lenv > lenu {
        return false;
    }

    
    for i in 0..=lenu-lenv{
        if i + lenv <= lenu {
            if second_list == &first_list[i..i+lenv]{
                return true;
            }
        }
    
    }

    false

}

pub fn is_sublist(first_list:&[i32], second_list:&[i32]) -> bool{
    let  lenv = second_list.len() as usize; // large
    let  lenu = first_list.len() as usize; // small
    if lenu > lenv {
        return false;
    }

    
    for i in 0..=lenv-lenu{
        if i + lenu <= lenv {
            if first_list == &second_list[i..i+lenu]{
                return true;
            }
        }
    
    }

    false

}



pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
      
    let is_equalv = is_equal(first_list, second_list);
    let is_sublistv = is_sublist(first_list, second_list);
    let is_superlistv = is_superlist(first_list, second_list);


    if is_equalv {
        return Comparison::Equal;
    } else if is_sublistv {
        return Comparison::Sublist;
    } else if is_superlistv{
        return Comparison::Superlist;
    } 

    return Comparison::Unequal;

     
}
