// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    direction: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self { 
        Robot {direction: d, x, y} }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.direction {
            // => Robot::new(direction, self.x, self.y),
            Direction::North => Self { direction: Direction::East,  ..self },
            Direction::East  => Self { direction: Direction::South, ..self },
            Direction::South => Self { direction: Direction::West,  ..self },
            Direction::West  => Self { direction: Direction::North, ..self }
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => Self { direction: Direction::West,  ..self },
            Direction::East  => Self { direction: Direction::North, ..self },
            Direction::South => Self { direction: Direction::East,  ..self },
            Direction::West  => Self { direction: Direction::South, ..self }
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Self { y: self.y + 1, ..self },
            Direction::East  => Self { x: self.x + 1, ..self },
            Direction::South => Self { y: self.y - 1, ..self },
            Direction::West  => Self { x: self.x - 1, ..self }
        }
    }

    #[must_use]
    // instructions like: "LAAARRRALLLL"
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, cmd| {
            match cmd {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => robot,
            }
        })
    }
    // pub fn instructions(self, instructions: &str) -> Self {
    //     let mut robot = self;
    //     for i in instructions.chars() {
    //         match i {
    //             'R' => robot = robot.turn_right(),
    //             'A' => robot = robot.advance(),
    //             'L' => robot = robot.turn_left(),
    //              _ => panic!("")
    //         }
    //     }
    //     robot
    // }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    // why return &Direction ?
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
