use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};
use stdweb::traits::*;

pub struct Canvas {
    canvas: CanvasElement,
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    scale_w: u32,
    scale_h: u32
}

impl Canvas {
    pub fn new(id: &str, width: u32, height: u32) -> Self {
        let canvas: CanvasElement = document()
            .query_selector(id).unwrap().unwrap()
            .try_into().unwrap();
        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let scale_h = canvas.height()/height;
        let scale_w = canvas.width()/width;
        Canvas {
            canvas,
            ctx,
            width,
            height,
            scale_h,
            scale_w
        }
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        self.ctx.set_fill_style_color(color);

        let x = x as f64 * self.scale_w as f64;
        let y = y as f64 * self.scale_h as f64;

        self.ctx.fill_rect(x, y, self.scale_w as f64, self.scale_h as f64);
    }

    pub fn flush(&self, color: &str) {
        self.ctx.set_fill_style_color(color);
        self.ctx.fill_rect(0.0, 0.0,
            self.width as f64 * self.scale_w as f64,
            self.height as f64 * self.scale_h as f64
        );
    }


}