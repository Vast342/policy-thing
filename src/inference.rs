use std::ops::{AddAssign, Div, Mul};

use crate::{arch::{INPUT_SIZE, OUTPUT_SIZE}, prng::fill_array, types::{datapoint::Datapoint, piece::Piece}};
const OW_SIZE: usize = INPUT_SIZE * OUTPUT_SIZE;
#[derive(Clone, Copy, Debug)]
pub struct PolicyNetwork{
    pub output_weights: [f32; OW_SIZE],
    pub output_biases: [f32; OUTPUT_SIZE],
}
impl AddAssign for Box<PolicyNetwork> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..(INPUT_SIZE * OUTPUT_SIZE) {
            self.output_weights[i] += rhs.output_weights[i];
        }
        for i in 0..OUTPUT_SIZE {
            self.output_biases[i] += rhs.output_biases[i];
        }
    }
}

impl Div<f32> for Box<PolicyNetwork> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let mut net = Box::new(PolicyNetwork::empty());
        for i in 0..(INPUT_SIZE * OUTPUT_SIZE) {
            net.output_weights[i] = self.output_weights[i] / rhs;
        }
        for i in 0..OUTPUT_SIZE {
            net.output_biases[i] = self.output_biases[i] / rhs;
        }
        net
    }
}
impl Mul<f32> for Box<PolicyNetwork> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut net = Box::new(PolicyNetwork::empty());
        for i in 0..(INPUT_SIZE * OUTPUT_SIZE) {
            net.output_weights[i] = self.output_weights[i] * rhs;
        }
        for i in 0..OUTPUT_SIZE {
            net.output_biases[i] = self.output_biases[i] * rhs;
        }
        net
    }
}
impl PolicyNetwork {
    pub fn rand() -> Self {
        Self{output_weights: fill_array(), output_biases: fill_array()}
    }
    pub const fn empty() -> Self {
        Self{output_weights: [0.0; INPUT_SIZE * OUTPUT_SIZE], output_biases: [0.0; OUTPUT_SIZE]}
    }
    pub fn add(&mut self, other: &Box<Self>) {
        for i in 0..(INPUT_SIZE * OUTPUT_SIZE) {
            self.output_weights[i] += other.output_weights[i];
        }
        for i in 0..OUTPUT_SIZE {
            self.output_biases[i] += other.output_biases[i];
        }
    }
}
/* will need for more layers but not rn with my glorified psqts
pub struct PolicyNetworkState{

}*/

pub const PIECE_STEP: usize = 64;
pub const COLOR_STEP: usize = 64 * 6;

pub fn calculate_index(move_piece: Piece, move_to: usize, piece: Piece, square: usize) -> usize {
    //println!("mp: {}, mt: {}, p: {}, s: {}", move_piece, move_to, piece, square);
    let move_number  = PIECE_STEP * move_piece.piece() as usize + move_to;
    let input_number = COLOR_STEP * piece.color() as usize + PIECE_STEP * piece.piece() as usize + square;
    let thing = INPUT_SIZE * move_number + input_number;
    //assert!(thing < 294912, "fuck {} {} {} {}", move_piece, move_to, piece, square);
    thing
    // highest possible would be uhhhh
    // 768 * (64 * 5 + 63) + (384 + 64 * 5 + 63)
}

pub fn get_gradient(og_point: Datapoint, network: &Box<PolicyNetwork>, gradient: &mut Box<PolicyNetwork>) {
    let mut point = og_point;
    let total_visits: f32 = {
        let mut sum = 0.0;
        for i in 0..32 {
            sum += point.moves[i].1 as f32;
        }
        sum
    };
    // convert position into mailbox   (no piece)
    let mut mailbox = [Piece(6); 64];
    let mut piece_pairs = 0;
    let mut piece_count = 0;
    while point.occupied.is_not_empty() {
        let square = point.occupied.pop_lsb() as usize;
        mailbox[square] = point.pieces[piece_pairs].nth(piece_count);
        piece_count += 1;
        if piece_count == 2 {
            piece_pairs += 1;
            piece_count = 0;
        }
    }
    // for each move
    for i in 0..32 {
        let (mov, visits) = point.moves[i];
        if visits != 0 {
            // get piece-to
            let piece = mailbox[mov.from() as usize];
            if piece.piece() < 6 {
                let to = mov.to();
                // infer
                let mut result = network.output_biases[(64 * piece.0 + to) as usize];
                for piece_index in 0..64 {
                    let this_piece = mailbox[piece_index];
                    if this_piece != Piece(6) {
                        let index = calculate_index(piece, to as usize, this_piece, piece_index);
                        result += network.output_weights[index];
                    }
                }
                // loss
                let loss = result - (visits as f32 / total_visits);
                gradient.output_biases[(64 * piece.0 + to) as usize] -= loss;
                for piece_index in 0..64 {
                    let this_piece = mailbox[piece_index];
                    if this_piece != Piece(6) {
                        let index = calculate_index(piece, to as usize, this_piece, piece_index);
                        gradient.output_weights[index] -= loss;
                    }
                }
            } else { break; }
        }
    }
}

pub fn get_loss(og_point: Datapoint, network: &PolicyNetwork) -> f32 {
    let mut point = og_point;
    let total_visits: f32 = {
        let mut sum = 0.0;
        for i in 0..32 {
            sum += point.moves[i].1 as f32;
        }
        sum
    };
    // convert position into mailbox   (no piece)
    let mut mailbox = [Piece(6); 64];
    let mut piece_pairs = 0;
    let mut piece_count = 0;
    while point.occupied.is_not_empty() {
        let square = point.occupied.pop_lsb() as usize;
        mailbox[square] = point.pieces[piece_pairs].nth(piece_count);
        piece_count += 1;
        if piece_count == 2 {
            piece_pairs += 1;
            piece_count = 0;
        }
    }
    let mut sum_loss = 0.0;
    // for each move
    for i in 0..32 {
        let (mov, visits) = point.moves[i];
        // get piece-to
        let piece = mailbox[mov.from() as usize];
        if piece.piece() != 6 {
            let to = mov.to();
            // infer
            let mut result = network.output_biases[(64 * piece.0 + to) as usize];
            for piece_index in 0..64 {
                let this_piece = mailbox[piece_index];
                if this_piece != Piece(6) {
                    let index = calculate_index(piece, to as usize, this_piece, piece_index);
                    result += network.output_weights[index];
                }
            }
            // loss
            let loss = result - (visits as f32 / total_visits);
            sum_loss += loss;
        }
    }
    sum_loss
}