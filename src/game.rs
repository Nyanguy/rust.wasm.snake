use crate::snake::Snake;
use crate::snake::Food;
use crate::canvas::Canvas;

pub struct Game {
    snake: Snake,
    food: Food,
    canvas: Canvas,
}

impl Game {
    pub fn draw(&self) {
        self.canvas.flush("white");
        self.canvas.draw(self.snake.head.0, self.snake.head.1, "green");
    }
}