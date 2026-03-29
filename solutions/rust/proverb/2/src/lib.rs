pub fn text<'a>(list: &'a [&'a str]) -> Vec<Vec<&'a str>> {
    let len = list.len();
    let mut var: Vec<Vec<&'a str>> = Vec::new();

    if len == 0 {
        return var;
    }

    let first = list[0];

    for i in 0..len - 1 {
        var.push(vec![list[i], list[i + 1]]);
    }

    var.push(vec![first]);
    var
}

pub fn gen_verse<'a >(s1:&str, s2:&str) -> String{
    format!("For want of a {s1} the {s2} was lost.")
}


pub fn build_proverb(list: &[&str]) -> String {
    let mut verse = String::new();
    if list.is_empty(){
        return "".to_string();
    } else {
        let first: &str = &list[0];
        for i in list {
            let a: &str = &list[0];
            let b: &str = &list[1];
            verse.push_str(&gen_verse(a, b));
        }
    verse.push_str("\nAnd all for the want of a {first}.")
    }
    verse

}
