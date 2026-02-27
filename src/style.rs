use crate::{Direction, Margin, Size};

/// Style of Node
#[derive(Default, Clone)]
pub struct Style {
    pub size: Size,
    pub gap: Size,
    pub direction: Direction,
    pub margin: Margin,
}
