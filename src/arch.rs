// piece of a color on a square tuples
pub const INPUT_SIZE: usize = 768;
// piece-to moves
pub const OUTPUT_SIZE: usize = 384;

pub const BATCH_SIZE: usize = 128;
pub const NUM_SUPERBATCHES: usize = 100;

pub const BATCHES_PER_SUPERBATCH: usize = 1024;
pub const POS_PER_SUPERBATCH: usize = BATCHES_PER_SUPERBATCH * BATCH_SIZE;
pub const TOTAL_POSITIONS: usize = POS_PER_SUPERBATCH * NUM_SUPERBATCHES;

pub const START_LR: f32 = 0.001;
pub const END_LR: f32 = 0.00001;

pub fn lr(superbatch: usize) -> f32 {
    let slope = (END_LR - START_LR) / NUM_SUPERBATCHES as f32;
    START_LR + slope * superbatch as f32
}
