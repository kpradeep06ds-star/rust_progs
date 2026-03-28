pub fn square(s: u32) -> u64 {
    // 2**0 + 2**1 + 2**2 + 2**3 ...
    (2 as u64).pow(s-1)
}

pub fn total() -> u64 {
    let mut sums: u64 = 0 ;
    for i in 1..=64{
        sums += square(i);
    }
    sums
}
