#[derive(Debug)]
pub struct HighScores<'a> {
    s: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(v: &'a [u32]) -> Self {
        HighScores { s: v }
    }
    pub fn scores(&self) -> &[u32] {
        self.s
    }
    pub fn latest(&self) -> Option<u32> {
        if self.s.is_empty() {
            None
        } else {
            Some(self.s[self.s.len() - 1])
        }
    }
    pub fn personal_best(&self) -> Option<u32> {
        if self.s.is_empty() {
            return None;
        }
        let mut m = self.s[0];

        for &x in self.s {
            if x > m {
                m = x;
            }
        }
        Some(m)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut a = self.s.to_vec();
        let n = a.len();
        let mut i = 0;

        while i < n {
            let mut j = 0;
            while j + 1 < n {
                if a[j] < a[j + 1] {
                    let t = a[j];
                    a[j] = a[j + 1];
                    a[j + 1] = t;
                }
                j += 1;
            }
            i += 1;
        }

        let mut r = Vec::new();
        let mut k = 0;
        while k < 3 && k < a.len() {
            r.push(a[k]);
            k += 1;
        }
        r
    }
}
