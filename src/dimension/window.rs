use crate::dimension::Size;
use crate::dimension::get_window_size;

#[derive(Debug, Clone)]
pub struct Window {
    pub size: Size,
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

impl Window {
    pub fn new() -> Self {
        Window {
            size: get_window_size(),
        }
    }

    pub fn size(&self) -> Size {
        self.size.clone()
    }

    pub fn update_size(&mut self) {
        self.size = get_window_size();
    }
}
