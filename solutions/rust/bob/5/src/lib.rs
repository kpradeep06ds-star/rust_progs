pub fn ask_qn(s:&str) -> bool{
    s.trim_end().ends_with('?')
}

pub fn all_caps(s: &str) -> bool {
    let mut has_letter = false;

    for i in s.chars() {
        if i.is_alphabetic() {
            has_letter = true;

            if !i.is_uppercase() {
                return false;
            }
        }
    }

    has_letter
}

// pub fn all_caps(s: &str) -> bool {
//     let letters: Vec<char> = s.chars().filter(|c| c.is_alphabetic()).collect();
//     !letters.is_empty() && letters.iter().all(|c| c.is_uppercase())
// }

pub fn yell_question(s:&str) -> bool{
    ask_qn(s) && all_caps(s)    
}

pub fn all_empty(s:&str) -> bool{
    s.trim().is_empty()
}

pub fn reply(message: &str) -> &str {
    let m = message;

    if all_empty(m) {
        "Fine. Be that way!"
    } else if yell_question(m) {
        "Calm down, I know what I'm doing!"
    } else if all_caps(m) {
        "Whoa, chill out!"
    } else if ask_qn(m) {
        "Sure."
    } else {
        "Whatever."
    }

}
