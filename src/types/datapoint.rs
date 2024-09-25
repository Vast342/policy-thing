use super::{bitboard::Bitboard, moves::Move, piece::Piece};


#[derive(Clone, Copy, Debug)]
pub struct PiecePair(pub u8);
impl PiecePair {
    pub fn first(&self) -> Piece {
        Piece(self.0 >> 4)
    }
    pub fn second(&self) -> Piece {
        Piece(self.0 & 0b1111)
    }
    pub fn nth(&self, n: usize) -> Piece {
        if n == 0 {
            self.first()
        } else if n == 1 {
            self.second()
        } else { panic!("tried getting a piece that don't exist") }
    }
}

#[derive(Clone, Copy, Debug)]
// size = 160 (0xA0), align = 0x8
pub struct Datapoint {
    pub occupied: Bitboard,
    // 4 bits per piece, in order of the occ's bits, lsb to msb
    pub pieces: [PiecePair; 16],
    // ctm, realistically it should be one bit but bruh
    pub ctm: u8,
    // number of visits on the root node is calculated from the sum of this array's visits
    // it's the 92 most visited moves out of however many the position has
    pub moves: [(Move, u16); 92],
}

impl Datapoint {
    pub fn new() -> Self {
        Self{occupied: Bitboard(0), pieces: [PiecePair(0); 16], ctm: 0, moves: [(Move::NULL_MOVE, 0); 92]}
    }
}