pub fn find_index(phrase: &str) -> Vec<char> {
    let chars: Vec<char> = phrase.chars().collect();
    let mut v = Vec::new();
    
    for i in 0..chars.len() {
        if i == 0 {
            v.push(chars[i]);
        } else if chars[i-1] == ' ' || chars[i-1] == '-' || chars[i].is_uppercase() {
            v.push(chars[i]);
        }
    }
    v
}

pub fn abbreviate(phrase: &str) -> String {
    let v:Vec<char> = find_index(phrase);
    println!("{:?}", v);
    let joined_string = v.into_iter().collect();
    joined_string
}
