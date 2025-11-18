pub fn reply(s: &str) -> &str {
    let t = s.trim();
    if t == "" {
        return "Fine. Be that way!";
    }

    let q = t.ends_with('?');
    let mut has = false;
    for c in t.chars() {
        if l(c) {
            has = true;
            break;
        }
    }

    let mut up = true;

    if has {
        for c in t.chars() {
            if l(c) && !c.is_uppercase() {
                up = false;
                break;
            }
        }
    } else {
        up = false;
    }
    if up && q {
        return "Calm down, I know what I'm doing!";
    }
    if up {
        return "Whoa, chill out!";
    }
    if q {
        return "Sure.";
    }
    "Whatever."
}

fn l(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c)
}
