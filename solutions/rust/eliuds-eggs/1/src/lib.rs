pub fn decimal_binary(num: u32) -> Vec<usize> {

    let mut v:Vec<usize> = Vec::new();
    let mut m:usize = num as usize;
    let mut rem: usize;
    loop {
        rem = m % 2; // 13 -> 1  , 6 -> 0, 3 -> 1, 1->1 
        m  = m / 2 ; // 6 , 3, 1
        v.push(rem); // v =[1, 0, 1]
        if m < 1{
            break;
        }
        
    }
    v
}


pub fn egg_count(display_value: u32) -> usize {

    //println!("{:?}", decimal_binary(display_value));

    //println!("count the eggs in {display_value}") ;
    let mut count:usize = 0;
    let val:Vec<usize> = decimal_binary(display_value);
    
    for i in val{
        if i == 1{
            count += 1;
        }
    }

    count

    
}
