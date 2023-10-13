#[derive(Debug)]

pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    // Queen 在棋盘上的位置是个合法位置, 即 i/j > 0 && i/j < 8
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some( Self { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        // Self(ChessPosition{rank: position.rank, file: position.file})
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.rank == other.0.rank ||
        self.0.file == other.0.file ||
        (self.0.file - other.0.file).abs() == (self.0.rank - other.0.rank).abs()
    }
}
