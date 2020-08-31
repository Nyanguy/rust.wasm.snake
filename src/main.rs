#[cfg(all(target_arch = "wasm32", feature = "stdweb"))]
#[macro_use]
extern crate stdweb;

mod movement;
mod canvas;
mod snake;

use canvas::Canvas;
use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, IEventTarget, document};

use std::rc::Rc;
use std::cell::RefCell;

use crate::snake::Snake;
use crate::movement::Direction;


fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<Canvas>) {
    stdweb::web::set_timeout(move||{
        snake.borrow_mut().update();
        snake.borrow().render(&canvas);
        game_loop(snake.clone(), canvas.clone());
    },100);
}


fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 20, 20);
    let mut snake = Rc::new(RefCell::new(Snake::new(20, 20)));

    document().add_event_listener({
        let snake = snake.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => snake.borrow_mut().change_direction(Direction::Left),
                "ArrowRight" => snake.borrow_mut().change_direction(Direction::Right),
                "ArrowUp" => snake.borrow_mut().change_direction(Direction::Up),
                "ArrowDown" => snake.borrow_mut().change_direction(Direction::Down),
                _ => {}
            };
        }
    });

    game_loop(snake, Rc::new(canvas));
}
