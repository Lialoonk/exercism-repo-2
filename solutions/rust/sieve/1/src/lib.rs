pub fn primes_up_to(n:u64)->Vec<u64>{
    let mut v=(2..=n).map(Some).collect::<Vec<_>>();
    (0..v.len()).filter_map(|i|{
        let p=v[i].take()?;
        (p..=n)
            .step_by(p as usize)
            .skip(1)
            .for_each(|k| v[(k-2) as usize]=None);
        Some(p)
    }).collect()
}
