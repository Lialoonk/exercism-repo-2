use std::collections::BTreeMap;

#[allow(clippy::new_without_default)]
pub struct School(BTreeMap<u32, Vec<String>>);
impl School {
    pub fn new() -> School{
        School(BTreeMap::new())
    }

    pub fn add(&mut self, g: u32, n: &str){
        let s = n.to_string();
        if self.0.values().any(|l| l.contains(&s)){
            return;
        }
        self.0
            .entry(g)
            .and_modify(|l| l.push(s.clone()))
            .or_insert(vec![s]);
    }

    pub fn grades(&self) -> Vec<u32>{
        self.0.keys().cloned().collect()
    }
    pub fn grade(&self, g: u32) -> Vec<String>{
        match self.0.get(&g){
            Some(l) => {
                let mut x = l.clone();
                x.sort();
                x
            }
            None => vec![],
        }
    }
}
