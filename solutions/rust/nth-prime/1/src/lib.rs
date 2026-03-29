pub fn is_prime(n: u32) -> bool{
    if n <= 1 {
        return false;
    } else if n == 2{
        return true;
    } else {
        let urange : u32 = n.isqrt() + 1 as u32;
        for i in 2..=urange{
            if n.is_multiple_of(i){
                return false;
            }
        }
        true
    }
}

pub fn nth(n: u32) -> u32 {

    
    let mut v: Vec<u32> = Vec::new();
    let mut m = 2; // first prime

    while v.len() <= n as usize{
            if is_prime(m){
                v.push(m);
            }
            m += 1;
        }
    v[n as usize]
    
}
