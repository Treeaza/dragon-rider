use std::ops;
use std::fmt;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn add(self, _rhs: Vector2i) -> Vector2i {
        Vector2i { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl ops::Sub<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn sub(self, _rhs: Vector2i) -> Vector2i {
        Vector2i { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}

impl ops::AddAssign<Vector2i> for Vector2i {
    fn add_assign(&mut self, _rhs: Vector2i) {
        *self = Vector2i {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        };
    }
}

impl ops::SubAssign<Vector2i> for Vector2i {
    fn sub_assign(&mut self, _rhs: Vector2i) {
        *self = Vector2i {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y
        };
    }
}

impl ops::Mul<i32> for Vector2i {
    type Output = Vector2i;

    fn mul(self, _rhs: i32) -> Vector2i {
        Vector2i { x: self.x * _rhs, y: self.y * _rhs}
    }
}

impl fmt::Display for Vector2i {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TurnDirection {
    LEFT,
    RIGHT
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

impl fmt::Display for Direction {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = String::from(match *self {
            Direction::NORTH => "north",
            Direction::SOUTH => "south",
            Direction::EAST => "east",
            Direction::WEST => "west",
        });
        write!(f, "{}", s)
    }
}

impl fmt::Display for TurnDirection {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = String::from(match *self {
            TurnDirection::LEFT => "L",
            TurnDirection::RIGHT => "R",
        });
        write!(f, "{}", s)
    }
}

impl TurnDirection {
    pub fn opposite(&self) -> TurnDirection {
        match *self {
            TurnDirection::LEFT => TurnDirection::RIGHT,
            TurnDirection::RIGHT => TurnDirection::LEFT,
        }
    }
}

impl Direction {
    pub fn next_direction (&self, change: TurnDirection) -> Direction {
        match *self {
            Direction::NORTH => if TurnDirection::LEFT == change {Direction::WEST} else {Direction::EAST},
            Direction::SOUTH => if TurnDirection::LEFT == change {Direction::EAST} else {Direction::WEST},
            Direction::EAST => if TurnDirection::LEFT == change {Direction::NORTH} else {Direction::SOUTH},
            Direction::WEST => if TurnDirection::LEFT == change {Direction::SOUTH} else {Direction::NORTH},
        }
    }

    pub fn cartesian_move (&self) -> Vector2i {
        match *self {
            Direction::NORTH => Vector2i { x: 0, y: 1 },
            Direction::SOUTH => Vector2i { x: 0, y: -1 },
            Direction::EAST => Vector2i { x: 1, y: 0 },
            Direction::WEST => Vector2i { x: -1, y: 0 },
        }
    }
}
