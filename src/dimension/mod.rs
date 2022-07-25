pub mod canvas;
pub mod geo;
pub mod point;
pub mod window;

use crate::utils::{
    get_window,
    f64_from_js,
};

#[derive(Debug, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Size { width, height }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::new(0.0, 0.0)
    }
}

#[derive(Debug, Clone)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl Range {
    pub fn new(min: f64, max: f64) -> Self {
        Range { min, max }
    }
}

impl Default for Range {
    fn default() -> Self {
        Range::new(0.0, 0.0)
    }
}

pub fn get_window_size() -> Size {
    match get_window() {
        Ok(win) => Size::new(
            win.inner_width().map_or(0_f64, f64_from_js),
            win.inner_height().map_or(0_f64, f64_from_js),
        ),
        Err(_) => Size::new(0_f64, 0_f64),
    }
}
