use crate::{Direction, Margin, Size};

/// Style of Node
#[derive(Default, Clone, Debug)]
pub struct Style {
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub gap: Size,
    pub direction: Direction,
    pub margin: Margin,
}
