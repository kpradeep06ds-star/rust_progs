pub fn ask_qn(s:&str) -> bool{
    s.trim_end().ends_with('?')
}

pub fn all_caps(s:&str) -> bool{
    for i in s.chars(){
        if !i.is_uppercase() && i.is_alphabetic(){
            return false;
        } 
    }
    true
}

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
