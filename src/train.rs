use crate::{arch::{lr, BATCHES_PER_SUPERBATCH, BATCH_SIZE, NUM_SUPERBATCHES, POS_PER_SUPERBATCH}, dataloader::Loader, inference::{get_gradient, PolicyNetwork}};
use std::{fs::File, io::{BufWriter, Write}, time::Instant};

pub fn train() {
    // init network
    let mut net = Box::new(PolicyNetwork::rand());
    // data loader
    let mut loader = Loader::new();
    let start = Instant::now();
    // train
    for superbatch_num in 0..NUM_SUPERBATCHES {
        let lr = lr(superbatch_num);
        for batch_num in 0..BATCHES_PER_SUPERBATCH {
            let batch_start = Instant::now();
            let mut gradient_sum = Box::new(PolicyNetwork::empty());
            for _position_num in 0..BATCH_SIZE {
                let point = loader.get_position();
                let gradient = get_gradient(point, &net);
                gradient_sum += gradient;
            }
            net += gradient_sum / BATCH_SIZE as f32 * lr;
            println!("Batch {} done | {} pos/sec", batch_num + 1, BATCH_SIZE as f32 / batch_start.elapsed().as_secs_f32());
            dbg!(net.output_weights[0]);
        }
        println!("Superbatch {} done | {} pos/sec", superbatch_num + 1, (POS_PER_SUPERBATCH * (superbatch_num + 1)) as f32 / start.elapsed().as_secs_f32());
    }
    // save to a file
    let mut writer = BufWriter::new(File::create("apn_001.pn").expect("couldn't create file"));
    unsafe { writer.write_all(any_as_u8_slice(&net)).expect("failed to write to file"); }
}

// thank you stack overflow :)
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}