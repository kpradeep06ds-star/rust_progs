pub fn search(arrayn: &[i32], key:i32) -> Option<usize>{

    if arrayn.is_empty(){
        return None;
    }

    let len:usize = arrayn.len();
    let split = len/2;
    let mid_value = arrayn[split];

    let left = &arrayn[0..=split] ;
    let right = &arrayn[split..len];

    if key == mid_value{
        return Some(split);
    } else if key <= *left.get(split).unwrap(){
        search(left, key)
    } else {
        search(right, key).map(|index| index + split + 1)
    } 
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // let mut sorted_copy = array.to_vec(); // This clones the data
    // sorted_copy.sort();
    search(&array, key)

}
