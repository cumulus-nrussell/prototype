use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use lazy_static::lazy_static;

use crate::{board::Board, direction::Direction, game_error::GameError, piece::Piece};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: i8, y: i8) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    pub fn new_i32(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn new_i8(x: i8, y: i8) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }

    // this implements "odd-r horizontal" which offsets odd rows to the right
    pub fn direction(&self, to: &Position) -> Direction {
        // even rows
        if self.y.rem_euclid(2) == 0 {
            return match (to.x - self.x, to.y - self.y) {
                (-1, -1) => Direction::NW,
                (0, -1) => Direction::NE,
                (1, 0) => Direction::E,
                (0, 1) => Direction::SE,
                (-1, 1) => Direction::SW,
                (-1, 0) => Direction::W,
                // This panic is okay, because if it ever gets called with an invalid move, it
                // implies there is a problem with the engine itself, not with user input
                (x, y) => {
                    panic!("(even) Direction of movement unknown, from: {self} to: {to} ({x},{y})")
                }
            };
        }
        // odd rows
        match (to.x - self.x, to.y - self.y) {
            (0, -1) => Direction::NW,
            (1, -1) => Direction::NE,
            (1, 0) => Direction::E,
            (1, 1) => Direction::SE,
            (0, 1) => Direction::SW,
            (-1, 0) => Direction::W,
            // This panic is okay, because if it ever gets called with an invalid move, it
            // implies there is a problem with the engine itself, not with user input
            (x, y) => {
                panic!("(odd) Direction of movement unknown, from: {self} to: {to} ({x},{y})")
            }
        }
    }

    pub fn common_adjacent_positions(&self, to: &Position) -> (Position, Position) {
        let (dir1, dir2) = self.direction(to).adjacent_directions();
        (self.to(&dir1), self.to(&dir2))
    }

    pub fn to(&self, direction: &Direction) -> Position {
        // even rows
        if self.y.rem_euclid(2) == 0 {
            return match direction {
                Direction::NW => Position::new_i32(self.x - 1, self.y - 1),
                Direction::NE => Position::new_i32(self.x, self.y - 1),
                Direction::E => Position::new_i32(self.x + 1, self.y),
                Direction::SE => Position::new_i32(self.x, self.y + 1),
                Direction::SW => Position::new_i32(self.x - 1, self.y + 1),
                Direction::W => Position::new_i32(self.x - 1, self.y),
            };
        }
        // odd rows
        match direction {
            Direction::NW => Position::new_i32(self.x, self.y - 1),
            Direction::NE => Position::new_i32(self.x + 1, self.y - 1),
            Direction::E => Position::new_i32(self.x + 1, self.y),
            Direction::SE => Position::new_i32(self.x + 1, self.y + 1),
            Direction::SW => Position::new_i32(self.x, self.y + 1),
            Direction::W => Position::new_i32(self.x - 1, self.y),
        }
    }

    pub fn from_string(s: &str, board: &Board) -> Result<Position, GameError> {
        if s.starts_with('.') {
            return Ok(Position::new(0, 0));
        }

        lazy_static! {
            static ref RE: Regex = Regex::new(r"([-/\\]?)([wb][ABGMLPSQ]\d?)([-/\\]?)")
                .expect("This regex should compile");
        }
        if let Some(cap) = RE.captures(s) {
            let piece: Piece = cap[2].parse()?;
            if let Some(mut position) = board.position(&piece) {
                if !cap[1].is_empty() {
                    match &cap[1] {
                        "\\" => {
                            position = position.to(&Direction::NW);
                        }
                        "-" => {
                            position = position.to(&Direction::W);
                        }
                        "/" => {
                            position = position.to(&Direction::SW);
                        }
                        any => {
                            return Err(GameError::InvalidDirection {
                                direction: any.to_string(),
                            })
                        }
                    }
                }
                if !cap[3].is_empty() {
                    match &cap[3] {
                        "/" => {
                            position = position.to(&Direction::NE);
                        }
                        "-" => {
                            position = position.to(&Direction::E);
                        }
                        "\\" => {
                            position = position.to(&Direction::SE);
                        }
                        any => {
                            return Err(GameError::InvalidDirection {
                                direction: any.to_string(),
                            })
                        }
                    }
                }
                return Ok(position);
            }
        }
        Err(GameError::ParsingError {
            found: s.to_string(),
            typ: "position".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_direction_and_to() {
        for position in [Position::new(0, 0), Position::new(0, 1)] {
            for direction in Direction::all() {
                let new_position = position.to(&direction);
                let opposite_direction = new_position.direction(&position);
                let inital_position = new_position.to(&opposite_direction);
                assert_eq!(position, inital_position);
            }
        }
    }
}
