use std::collections::HashSet;

pub fn multiples(limit: u32, factor: u32) -> Vec<u32> {
    let mut out = Vec::new();

    if factor == 0 {
        return out;
    }

    let mut current = factor;
    while current < limit {
        out.push(current);
        current += factor;
    }

    out
}


pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut seen: HashSet<u32> = HashSet::new();

    for &factor in factors {
        let vals = multiples(limit, factor);
        for v in vals {
            seen.insert(v);
        }
    }

    seen.iter().sum()
}