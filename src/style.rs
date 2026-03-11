use crate::{Direction, HorizontalAlign, Margin, Padding, Size, VerticalAlign};

/// Style of Node
#[derive(Default, Clone, Copy, Debug)]
#[cfg_attr(feature = "types_stable_abi", repr(C))]
pub struct Style {
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub gap: Size,
    pub direction: Direction,
    pub margin: Margin,
    pub padding: Padding,
    pub vertical_align: VerticalAlign,
    pub horizontal_align: HorizontalAlign,
}
