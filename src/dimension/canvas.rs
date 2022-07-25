use web_sys::{
    CanvasRenderingContext2d,
    DomRect,
    HtmlCanvasElement,
};

use crate::dimension::Size;
use crate::utils::{
    device_pixel_ratio,
    lazy_round,
};

#[derive(Debug, Clone)]
pub struct Canvas {
    pub dpr: f64,
    pub el: HtmlCanvasElement,
    pub ctx: CanvasRenderingContext2d,
    pub width: f64,
    pub height: f64,
}

impl Canvas {
    pub fn new(
        el: HtmlCanvasElement,
        ctx: CanvasRenderingContext2d,
    ) -> Self {
        let dpr: f64 = device_pixel_ratio();
        ctx.scale(dpr, dpr).unwrap_or(());

        Canvas {
            dpr,
            el,
            ctx,
            width: 0_f64,
            height: 0_f64,
        }
    }

    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    pub fn update_size(&mut self) {
        let (w, h): (f64, f64) = get_canvas_size(&self.el);
        let width: f64 = w * self.dpr;
        let height: f64 = h * self.dpr;

        self.el.set_width(width as u32);
        self.el.set_height(height as u32);
        self.width = lazy_round(width);
        self.height = lazy_round(height);
    }
}

fn get_canvas_size(el: &HtmlCanvasElement) -> (f64, f64) {
    let rect: DomRect = el.get_bounding_client_rect();
    (rect.right() - rect.left(), rect.bottom() - rect.top())
}
