pub fn build_proverb(w: &[&str]) -> String {
    if w.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let mut i = 0;

    while i + 1 < w.len() {
        let a = w[i];
        let b = w[i + 1];
        out.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            a, b
        ));
        i += 1;
    }

    out.push_str(&format!(
        "And all for the want of a {}.",
        w[0]
    ));
    out
}
