use std::collections::{HashMap, HashSet};

pub fn solve(e: &str) -> Option<HashMap<char, u8>>{
    let mut t = e.split(" == ");
    let l: Vec<&str> = t.next().unwrap().split(" + ").collect();
    let r = t.next().unwrap();
    let mut u = Vec::new();
    let mut s = HashSet::new();

    for c in e.chars() {
        if c.is_alphabetic()&& s.insert(c){
            u.push(c);
        }
    }
    if u.len() > 10 {
        return None;
    }
    let mut z = [false; 10];
    let mut m = HashMap::new();
    if f(0, &u, &mut z, &mut m, &l, r){
        Some(m)
    } else {
        None
    }
}
fn f(
    i: usize,
    u: &[char],
    z: &mut [bool; 10],
    m: &mut HashMap<char, u8>,
    l: &[&str],
    r: &str,
) -> bool {
    if i == u.len() {
        return v(l, r, m);
    }
    let c = u[i];
    for d in 0..10 {
        if z[d] {
            continue;
        }
        if d == 0 && q(c, l, r) {
            continue;
        }
        z[d] = true;
        m.insert(c, d as u8);
        if f(i + 1, u, z, m, l, r) {
            return true;
        }
        z[d] = false;
        m.remove(&c);
    }
    false
}

fn q(c: char, l: &[&str], r: &str) -> bool {
    if r.starts_with(c) {
        return true;
    }
    for w in l {
        if w.starts_with(c) {
            return true;
        }
    }
    false
}
fn v(l: &[&str], r: &str, m: &HashMap<char, u8>) -> bool {
    let mut s = 0u64;
    for w in l {
        if let Some(x) = n(w, m) {
            s += x;
        } else {
            return false;
        }
    }
    match n(r, m) {
        Some(x) => s == x,
        None => false,
    }
}
fn n(s: &str, m: &HashMap<char, u8>) -> Option<u64> {
    let mut x = 0u64;
    for c in s.chars() {
        x = x * 10 + *m.get(&c)? as u64;
    }
    Some(x)
}
