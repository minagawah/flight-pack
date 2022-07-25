#[derive(Debug, Clone)]
pub struct PointCoord {
    pub x: f64,
    pub y: f64,
}

impl PointCoord {
    pub fn new(x: f64, y: f64) -> Self {
        PointCoord { x, y }
    }
}

impl Default for PointCoord {
    fn default() -> Self {
        PointCoord::new(0.0, 0.0)
    }
}
