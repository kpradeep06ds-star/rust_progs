pub fn even_odd_compute(n: u64) -> u64{
    if n%2 == 0{
        n/2
    } else {
        n*3 + 1
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    let mut m = n;
    let mut count = 0;
    if n == 0{
        return None;
    }

    while m > 1{
        m = even_odd_compute(m);
        count += 1;
    }

    Some(count)

}
