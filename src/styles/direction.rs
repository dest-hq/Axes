/// The direction that childs will be display
#[derive(Clone, Debug)]
pub enum Direction {
    Row,
    Column,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Column
    }
}
