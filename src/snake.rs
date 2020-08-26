use stdweb::unstable::TryInto;
use crate::movement::Direction;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

pub struct Snake {
    head: Block,
    tail: Vec<Block>,
    size: u32,
    pos_x: u32,
    pos_y: u32,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction
}