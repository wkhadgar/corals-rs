use crate::vectors::Vector2;

pub struct Cell {
    pub(crate) id: u64,
    pub(crate) position: Vector2,
    pub(crate) velocity: Vector2,
    pub(crate) mass: f64,
}

impl Cell {
    pub fn new(id: u64, pos: Vector2) -> Self {
        Self {
            id,
            position: pos,
            velocity: Vector2::default(),
            mass: 1.0,
        }
    }

    pub fn pos(&self) -> Vector2 {
        self.position
    }
    pub fn react(&mut self) {
        self.position += self.velocity;
    }
}