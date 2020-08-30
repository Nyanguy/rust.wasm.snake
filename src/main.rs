extern crate stdweb;

mod movement;
mod canvas;
mod snake;
mod game;

use canvas::Canvas;

fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("canvas", 20, 20);
    canvas.draw(5, 5, "red");

    stdweb::event_loop();

}
