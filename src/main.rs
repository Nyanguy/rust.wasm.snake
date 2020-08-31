// #[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
// #[macro_use]
extern crate stdweb;

mod movement;
mod canvas;
mod snake;

use canvas::Canvas;
use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, IEventTarget, document, Window};

use crate::snake::Snake;
use crate::movement::Direction;


fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 20, 20);
    let mut snake = Snake::new(20, 20);

    document().add_event_listener({
        let mut snake = snake.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => snake.change_direction(Direction::Left),
                "ArrowRight" => snake.change_direction(Direction::Right),
                "ArrowUp" => snake.change_direction(Direction::Up),
                "ArrowDown" => snake.change_direction(Direction::Down),
                _ => {}
            };
        }
    });


    stdweb::js! {@(no_return)
        window.setInterval(@{snake.update()}, 1000);
    }
}
