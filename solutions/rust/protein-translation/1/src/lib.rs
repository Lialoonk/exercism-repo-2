pub fn translate(r: &str)->Option<Vec<&str>>{
    let mut v = Vec::new();
    for c in r.as_bytes().chunks(3){
        let p=decode(c)?;
        if p == "STOP"{
            break;
        }
        v.push(p);
    }
    Some(v)
}

fn decode(c: &[u8])->Option<&'static str>{
    match c {
        b"AUG" => Some("Methionine"),
        b"UUU" | b"UUC" => Some("Phenylalanine"),
        b"UUA" | b"UUG" => Some("Leucine"),
        b"UCU" | b"UCC" | b"UCA" | b"UCG" => Some("Serine"),
        b"UAU" | b"UAC" => Some("Tyrosine"),
        b"UGU" | b"UGC" => Some("Cysteine"),
        b"UGG" => Some("Tryptophan"),
        b"UAA" | b"UAG" | b"UGA" => Some("STOP"),
        _ => None,
    }
}
