

pub fn factors(n: u64) -> Vec<u64> {
    
    let mut nn : u64 = n ;
    let mut v: Vec<u64> = Vec::new();
    let mut i = 2;

    while i*i <= nn{
        while nn.is_multiple_of(i){
            v.push(i);
            nn /= i;
        }
        i += 1;
    }

    if nn > 1{
        v.push(nn);
    }
    v
}
