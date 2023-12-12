#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
pub struct TripleExprIdx(usize);

impl TripleExprIdx {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}
