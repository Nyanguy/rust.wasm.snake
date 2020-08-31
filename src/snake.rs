use stdweb::web::alert;
use crate::movement::Direction;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use crate::canvas::Canvas;



#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Block(u32, u32);

impl Block {
    pub fn new(max_w: u32, max_h: u32) -> Self {
        let mut rng = rand::thread_rng();
        Block(rng.gen_range(0, max_w), rng.gen_range(0, max_h))
    }

    pub fn new_relative(max_w: u32, max_h: u32, head: Block, others: &Vec<Block>) -> Self {
        let mut food = Block::new(max_w, max_h);
        while head == food || others.contains(&food) {
            food = Block::new(max_w, max_h);
        }
        food
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Food {
    Apple,
    Orange,
    Pineapple,
    Cherry
}

impl Distribution<Food> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Food {
        match rng.gen_range(0, 4) {
            0 => Food::Apple,
            1 => Food::Orange,
            2 => Food::Pineapple,
            _ => Food::Cherry,
        }
    }
}


impl Food {
    pub fn get_color(&self) -> &str {
        match self {
            Food::Apple => "red",
            Food::Orange => "orange",
            Food::Pineapple => "yellow",
            Food::Cherry => "purple"
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Snake {
    pub head: Block,
    pub tail: Vec<Block>,
    width: u32,
    height: u32,
    food_type: Food,
    food_pos: Block,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction,
    score: u32,
    pub running: bool
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        let head = Block::new(width, height);
        let tail= Vec::new();
        let food_pos = Block::new_relative(width, height, head, &tail);
        Snake {
            head,
            tail,
            width,
            height,
            food_type: rand::random(),
            food_pos,
            direction: None,
            next_direction: None,
            last_direction: Direction::Up,
            score: 0,
            running: true,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.direction.is_none() && !self.last_direction.is_opposite(direction) {
            self.direction = Some(direction)
        } else if self.direction.iter().any(|d| !d.is_opposite(direction)) {
            self.next_direction = Some(direction)
        }
    }

    pub fn update_score(&self) {
        stdweb::js! { @(no_return)
                document.getElementById("score").innerHTML = "Score: " + @{self.score};
            }
    }

    pub fn update(&mut self) {
        let direction = self.direction.unwrap_or(self.last_direction);
        self.last_direction = direction;

        let new_head = match direction {
            Direction::Up => Block(self.head.0 % self.width,
                                   self.head.1.checked_sub(1).unwrap_or(self.height - 1) % self.height),
            Direction::Down => Block(self.head.0 % self.width, (self.head.1 + 1) % self.height),
            Direction::Left => Block(self.head.0.checked_sub(1).unwrap_or(self.width - 1) % self.width,
                                     self.head.1 % self.height),
            Direction::Right => Block((self.head.0 + 1) % self.width, self.head.1 % self.height),
        };

        self.tail.insert(0, self.head);

        if self.tail.contains(&new_head) {
            self.running = false;
            alert(format!("Game Over! Your score: {}", self.score).as_str());
            *self = Snake::new(self.width, self.height);
            self.score = 0;
            self.update_score();
        }

        self.head = new_head;

        if self.head == self.food_pos {
            self.score += 1;
            self.food_type = rand::random();
            self.food_pos = Block::new_relative(self.width, self.height, self.head, &self.tail);
            self.update_score();
        } else {
            self.tail.pop();
        }
        self.direction = self.next_direction.take();
    }

    pub fn render(&self, canvas: &Canvas) {
        canvas.flush("white");
        canvas.draw(self.head.0, self.head.1, "green");
        for &Block(x, y) in &self.tail {
            canvas.draw(x, y, "lightgreen");
        }
        canvas.draw(self.food_pos.0, self.food_pos.1, self.food_type.get_color());
    }

}