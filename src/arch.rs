// piece of a color on a square tuples
pub const INPUT_SIZE: usize = 768;
// piece-to moves
pub const OUTPUT_SIZE: usize = 384;

pub const CHECKPOINT_FREQ: usize = 5;

pub const BATCH_SIZE: usize = 16384;
pub const BATCHES_PER_SUPERBATCH: usize = 1024;
pub const NUM_SUPERBATCHES: usize = 500;
pub const POS_PER_SUPERBATCH: usize = BATCHES_PER_SUPERBATCH * BATCH_SIZE;
pub const TOTAL_POSITIONS: usize = POS_PER_SUPERBATCH * NUM_SUPERBATCHES;

pub const START_LR: f32 = 1.0;
pub const END_LR: f32 = 0.01;

pub fn lr(superbatch: usize) -> f32 {
    let slope = (END_LR - START_LR) / NUM_SUPERBATCHES as f32;
    START_LR + slope * superbatch as f32
}
