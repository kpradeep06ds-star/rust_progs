/// Check a Luhn checksum.
pub fn _split(code: &str) -> Option<Vec<u32>> {
    let mut numbers: Vec<u32> = Vec::new();

    for c in code.chars() {
        if c == ' ' {
            continue;
        } else if let Some(d) = c.to_digit(10) {
            numbers.push(d);
        } else {
            return None;
        }
    }

    numbers.reverse();

    let mut v: Vec<u32> = Vec::with_capacity(numbers.len());

    for (idx, i) in numbers.into_iter().enumerate() {
        if idx % 2 != 0 {
            v.push(_check_double(i));
        } else {
            v.push(i);
        }
    }

    Some(v)
}

pub fn _check_double(num: u32) -> u32 {
    let newnum = num * 2;

    if newnum > 9 {
        newnum - 9
    } else {
        newnum
    }
}

pub fn multiply(code: &str) -> bool {
    let numbers = match _split(code) {
        Some(v) => v,
        None => return false,
    };

    if numbers.len() < 2 {
        return false;
    }

    let fnum: u32 = numbers.iter().sum();
    fnum.is_multiple_of(10)
}

pub fn is_valid(code: &str) -> bool {
    multiply(code)
}