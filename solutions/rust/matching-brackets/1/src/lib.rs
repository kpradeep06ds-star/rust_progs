

pub fn brackets_are_balanced(string: &str) -> bool {

    let mut stack: Vec<char> = Vec::new();
    // ([{}]) => ['(', '[', '{' ]
    for ch in string.chars(){
        match ch {

            '(' | '[' | '{' => stack.push(ch),
             
            ')'  => {
                let top = stack.pop();
                if top != Some('('){
                    return false;
                }
            },

            '}' => {
                let top = stack.pop();
                if top != Some('{'){
                    return false;
                }
            },

            ']' => {
                let top = stack.pop();
                if top != Some('['){
                    return false;
                }
            },

            _ => {}

        }
    }

    stack.is_empty()


}
