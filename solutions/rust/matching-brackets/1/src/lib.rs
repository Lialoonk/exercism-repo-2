pub fn brackets_are_balanced(s: &str) -> bool {
    let mut v: Vec<char> = Vec::new();
    for c in s.chars() {
        if c == '(' {
            v.push('(');
        } else if c == '[' {
            v.push('[');
        } else if c == '{' {
            v.push('{');
        }

        else if c == ')' {
            if v.len() == 0 {
                return false;
            }
            let t = v.pop().unwrap();
            if t != '(' {
                return false;
            }
        }

        else if c == ']' {
            if v.len() == 0 {
                return false;
            }
            let t = v.pop().unwrap();
            if t != '[' {
                return false;
            }
        }

        else if c == '}' {
            if v.len() == 0 {
                return false;
            }
            let t = v.pop().unwrap();
            if t != '{' {
                return false;
            }
        }
    }
    v.len() == 0
}
