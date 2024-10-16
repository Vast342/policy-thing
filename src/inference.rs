use crate::{
    arch::INPUT_SIZE,
    net::PolicyNetwork,
    types::{datapoint::Datapoint, piece::Piece},
};
/* will need for more layers but not rn with my glorified psqts
pub struct PolicyNetworkState{

}*/

pub const PIECE_STEP: usize = 64;
pub const COLOR_STEP: usize = 64 * 6;

pub fn calculate_index(move_piece: Piece, move_to: usize, piece: Piece, square: usize) -> usize {
    //println!("mp: {}, mt: {}, p: {}, s: {}", move_piece, move_to, piece, square);
    let move_number = PIECE_STEP * move_piece.piece() as usize + move_to;
    let input_number =
        COLOR_STEP * piece.color() as usize + PIECE_STEP * piece.piece() as usize + square;
    let thing = INPUT_SIZE * move_number + input_number;
    //assert!(thing < 294912, "fuck {} {} {} {}", move_piece, move_to, piece, square);
    thing
    // highest possible would be uhhhh
    // 768 * (64 * 5 + 63) + (384 + 64 * 5 + 63)
}

pub fn get_gradient(
    og_point: Datapoint,
    network: &Box<PolicyNetwork>,
    gradient: &mut Box<PolicyNetwork>,
) {
    let mut point = og_point;
    let total_visits: f32 = {
        let mut sum = 0.0;
        for i in 0..92 {
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
    let mut results = [0.0; 92];
    // for each move
    for i in 0..92 {
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
                results[i] = result;
            } else {
                break;
            }
        }
    }
    let mut result_sum = 0.0;
    // normalize
    for i in 0..92 {
        results[i] = results[i].exp();
        result_sum += results[i];
    }
    for i in 0..92 {
        results[i] /= result_sum;
    }
    // loss
    for i in 0..92 {
        let (mov, visits) = point.moves[i];
        if visits != 0 {
            // get piece-to
            let piece = mailbox[mov.from() as usize];
            if piece.piece() < 6 {
                let to = mov.to();
                let loss = results[i] - (visits as f32 / total_visits);
                gradient.output_biases[(64 * piece.0 + to) as usize] -= loss;
                for piece_index in 0..64 {
                    let this_piece = mailbox[piece_index];
                    if this_piece != Piece(6) {
                        let index = calculate_index(piece, to as usize, this_piece, piece_index);
                        gradient.output_weights[index] -= loss;
                    }
                }
            } else {
                break;
            }
        }
    }
}

pub fn get_loss(og_point: Datapoint, network: &Box<PolicyNetwork>) -> f32 {
    let mut point = og_point;
    let total_visits: f32 = {
        let mut sum = 0.0;
        for i in 0..92 {
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
    let mut results = [0.0; 92];
    // for each move
    for i in 0..92 {
        let (mov, _visits) = point.moves[i];
        // get piece-to
        let piece = mailbox[mov.from() as usize];
        if piece.piece() != 6 {
            let to = mov.to();
            // infer
            results[i] = network.output_biases[(64 * piece.0 + to) as usize];
            for piece_index in 0..64 {
                let this_piece = mailbox[piece_index];
                if this_piece != Piece(6) {
                    let index = calculate_index(piece, to as usize, this_piece, piece_index);
                    results[i] += network.output_weights[index];
                }
            }
        }
    }
    let mut result_sum = 0.0;
    // normalize
    for i in 0..92 {
        results[i] = results[i].exp();
        result_sum += results[i];
    }
    for i in 0..92 {
        results[i] /= result_sum;
    }
    // for each move
    for i in 0..92 {
        let (mov, visits) = point.moves[i];
        // get piece-to
        let piece = mailbox[mov.from() as usize];
        if piece.piece() != 6 {
            // loss
            let loss = results[i] - (visits as f32 / total_visits);
            sum_loss += loss;
        }
    }
    sum_loss
}
