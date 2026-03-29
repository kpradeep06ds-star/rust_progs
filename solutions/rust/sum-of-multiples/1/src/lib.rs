use std::collections::HashSet;
pub fn multiples(maxnum: u32, multiplier: u32) -> Vec<u32>{
    let mut range:Vec<u32> = Vec::new();
    for i in 1..maxnum {
        let m: u32 = i*multiplier;
        if m < maxnum{
            range.push(i*multiplier);
        }
    }
    range
}

pub fn combine(s1: HashSet<u32>, s2: HashSet<u32>) -> HashSet<u32> {
    let s: HashSet<u32> = s1.union(&s2).copied().collect();
    s
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    
    let f1 = factors[0];
    let f2 = factors[1];
    let mut s1: HashSet<u32> = HashSet::new();
    let mut s2: HashSet<u32> = HashSet::new();

    let v1:Vec<u32> = multiples(limit, f1);
    let v2:Vec<u32> = multiples(limit, f2);

    for v in v1{
        s1.insert(v);
    }

    for v in v2{
        s2.insert(v);
    }

    let s: HashSet<u32> = combine(s1, s2);
    let total_sum = s.iter().sum::<u32>();

    total_sum

}
