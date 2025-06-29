pub struct LabelGenerator {
    pos: usize,
}

impl LabelGenerator {
    pub fn new() -> Self {
        Self { pos: 0 }
    }
    pub fn next(&mut self, prefix: &str) -> String {
        self.pos += 1;
        format!("{}_{}", prefix, self.pos)
    }
}
