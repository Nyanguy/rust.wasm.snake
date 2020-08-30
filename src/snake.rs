use stdweb::unstable::TryInto;
use crate::movement::Direction;
use rand::Rng;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

pub enum Food {
    Apple,
    Orange,
    Pineapple,
    Cherry
}

impl Food {
    fn get_color(&self) -> &str {
        match self {
            Food::Apple => "green",
            Food::Orange => "orange",
            Food::Pineapple => "yellow",
            Food::Cherry => "purple"
        }
    }
}


pub struct Snake {
    pub head: Block,
    pub tail: Vec<Block>,
    width: u32,
    height: u32,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction
}

impl Snake {
    pub fn new(width: u32, height: u32, field_w: u32, field_h: u32) -> Self {
        let mut rng = rand::thread_rng();
        pos_x = rng.gen_range(0, field_w);
        pos_y = rng.gen_range(0, field_h);

        Snake {
            head: Block(pos_x, pos_y),
            tail: Vec::new(),
            width,
            height,
            direction: None,
            next_direction: None,
            last_direction: Direction::Up
        }
    }

    pub fn check_overflow(&self, pos_x: u32, pos_y: u32) -> (u32, u32) {
        let mut new_x = pos_x;
        let mut new_y = pos_y;
        if pos_x < 0 {
            new_x = self.width;
        } else if pos_x >= self.width {
            new_x = 0;
        }
        if pos_y < 0 {
            new_y = self.height;
        } else if pos_y >= self.height {
            new_y = 0;
        }

        (new_x, new_y)
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.direction.is_none() {
            self.direction = Some(direction);
        } else if !self.direction.is_opposite(direction) {
            self.next_direction = Some(direction);
        }
    }

    pub fn update(&mut self) {
        let direction = self.direction.unwrap_or(self.last_direction);
        self.last_direction = direction;

        let new_head = match direction {
            Direction::Up => Block(self.head.0 % self.width, self.head.1.checked_sub(1) % self.height),
            Direction::Down => Block(self.head.0 % self.width, self.head.1 + 1 % self.height),
            Direction::Left => Block(self.head.0.checked_sub(1) % self.width, self.head.1 % self.height),
            Direction::Right => Block(self.head.0 + 1 % self.width, self.head.1 % self.height),
        };

        self.tail.insert(0, self.head);
        self.tail.pop();

        if self.tail.contains(&new_head) {
            // GAME OVER
        }

        self.head = new_head;
    }

}