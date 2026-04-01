
pub fn slice(digits: &str, len: usize) -> Vec<String> {

    
    let tlen = digits.len();

    let mut v: Vec<String> = Vec::new();

    if len > tlen {
        return v;
    }

    for i in 0..=tlen - len{
        if i + len <= tlen {
            v.push(digits[i..(i+len)].to_string());
        }
    }
    v
}

pub fn series(digits: &str, len: usize) -> Vec<String> {

    
        slice(digits, len)


}
