pub fn is_prime(n: u64) -> bool{
    if n <= 1 {
        return false;
    } else if n == 2{
        return true;
    } else {
        let urange : u64 = n.isqrt() + 1 as u64;
        for i in 2..=urange{
            if n.is_multiple_of(i){
                return false;
            }
        }
        true
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let m : u64 = n.isqrt()+1;
    let mut nn : u64 = n ;
    let mut v: Vec<u64> = Vec::new();
    for i in 2..=m{
        if is_prime(i) && nn.is_multiple_of(i){
            v.push(i);
            nn /= i;
        }
    }
    v
}
