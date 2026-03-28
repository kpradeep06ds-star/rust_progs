/// Check a Luhn checksum.
pub fn _split(code: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = code.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let num = numbers.len();
    
    let mut v:Vec<u32> = Vec::with_capacity(num);

    numbers.reverse();

    for (idx, i) in numbers.into_iter().enumerate(){
        if idx%2 != 0{
            v.push(_check_double(i));
        } else {
            v.push(i);
        }

    }

    v
}

pub fn _check_double(num: u32) -> u32{
    let newnum = num*2 as u32;
    
    let nine = 9 as u32 ;

    if newnum > nine{
        return newnum - nine;
    } else {
        return newnum;
    }
}


pub fn mulitply(code: &str) -> bool {
    let numbers: Vec<u32> = _split(code);

    let fnum: u32 = numbers.iter().sum();

    fnum % 10 == 0
}

pub fn is_valid(code: &str) -> bool {
    mulitply(code)
}
